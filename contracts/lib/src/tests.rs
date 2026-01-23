#[cfg(test)]
mod tests {
    use crate::propchain_contracts::PropertyRegistry;
    use crate::propchain_contracts::Error;
    use ink::primitives::AccountId;
    use propchain_traits::*;

    fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
        ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
    }

    fn set_caller(sender: AccountId) {
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
    }

    #[ink::test]
    fn new_works() {
        let contract = PropertyRegistry::new();
        assert_eq!(contract.property_count(), 0);
    }

    #[ink::test]
    fn register_property_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);

        let mut contract = PropertyRegistry::new();
        
        let metadata = PropertyMetadata {
            location: "123 Main St".to_string(),
            size: 1000,
            legal_description: "Test property".to_string(),
            valuation: 1000000,
            documents_url: "https://example.com/docs".to_string(),
        };

        let property_id = contract.register_property(metadata).expect("Failed to register property");
        assert_eq!(property_id, 1);
        assert_eq!(contract.property_count(), 1);

        let property = contract.get_property(property_id).unwrap();
        assert_eq!(property.owner, accounts.alice);
        assert_eq!(property.metadata.location, "123 Main St");
    }

    #[ink::test]
    fn transfer_property_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);

        let mut contract = PropertyRegistry::new();
        
        let metadata = PropertyMetadata {
            location: "123 Main St".to_string(),
            size: 1000,
            legal_description: "Test property".to_string(),
            valuation: 1000000,
            documents_url: "https://example.com/docs".to_string(),
        };

        let property_id = contract.register_property(metadata).expect("Failed to register property");
        
        // Transfer to bob
        set_caller(accounts.alice);
        assert!(contract.transfer_property(property_id, accounts.bob).is_ok());

        let property = contract.get_property(property_id).unwrap();
        assert_eq!(property.owner, accounts.bob);
    }

    #[ink::test]
    fn transfer_unauthorized_fails() {
        let accounts = default_accounts();
        set_caller(accounts.alice);

        let mut contract = PropertyRegistry::new();
        
        let metadata = PropertyMetadata {
            location: "123 Main St".to_string(),
            size: 1000,
            legal_description: "Test property".to_string(),
            valuation: 1000000,
            documents_url: "https://example.com/docs".to_string(),
        };

        let property_id = contract.register_property(metadata).expect("Failed to register property");
        
        // Try to transfer as charlie (not owner)
        set_caller(accounts.charlie);
        assert_eq!(contract.transfer_property(property_id, accounts.bob), Err(Error::Unauthorized));
    }

    #[ink::test]
    fn get_nonexistent_property_fails() {
        let contract = PropertyRegistry::new();
        assert_eq!(contract.get_property(999), None);
    }

    #[ink::test]
    fn update_metadata_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);

        let mut contract = PropertyRegistry::new();
        
        let metadata = PropertyMetadata {
            location: "123 Main St".to_string(),
            size: 1000,
            legal_description: "Test property".to_string(),
            valuation: 1000000,
            documents_url: "https://example.com/docs".to_string(),
        };

        let property_id = contract.register_property(metadata.clone()).expect("Failed to register");

        let new_metadata = PropertyMetadata {
            location: "123 Main St Updated".to_string(),
            size: 1100,
            legal_description: "Test property updated".to_string(),
            valuation: 1100000,
            documents_url: "https://example.com/docs/new".to_string(),
        };

        assert!(contract.update_metadata(property_id, new_metadata.clone()).is_ok());

        let property = contract.get_property(property_id).unwrap();
        assert_eq!(property.metadata, new_metadata);

        // Check event emission
        let events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert!(events.len() > 1); // Registration + Update
    }

    #[ink::test]
    fn update_metadata_unauthorized_fails() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        let metadata = PropertyMetadata {
            location: "123 Main St".to_string(),
            size: 1000,
            legal_description: "Test property".to_string(),
            valuation: 1000000,
            documents_url: "https://example.com/docs".to_string(),
        };
        let property_id = contract.register_property(metadata).expect("Failed to register");

        set_caller(accounts.bob);
        let new_metadata = PropertyMetadata {
            location: "123 Main St Updated".to_string(),
            size: 1100,
            legal_description: "Test property updated".to_string(),
            valuation: 1100000,
            documents_url: "https://example.com/docs/new".to_string(),
        };
        assert_eq!(contract.update_metadata(property_id, new_metadata), Err(Error::Unauthorized));
    }

    #[ink::test]
    fn approval_work() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        let metadata = PropertyMetadata {
            location: "123 Main St".to_string(),
            size: 1000,
            legal_description: "Test property".to_string(),
            valuation: 1000000,
            documents_url: "https://example.com/docs".to_string(),
        };
        let property_id = contract.register_property(metadata).expect("Failed to register");

        // Approve Bob
        assert!(contract.approve(property_id, Some(accounts.bob)).is_ok());
        assert_eq!(contract.get_approved(property_id), Some(accounts.bob));

        // Bob transfers property
        set_caller(accounts.bob);
        assert!(contract.transfer_property(property_id, accounts.charlie).is_ok());

        let property = contract.get_property(property_id).unwrap();
        assert_eq!(property.owner, accounts.charlie);

        // Approval should be cleared
        assert_eq!(contract.get_approved(property_id), None);
    }

    // Batch Operations Tests
    
    #[ink::test]
    fn batch_register_properties_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        let properties = vec![
            PropertyMetadata {
                location: "Property 1".to_string(),
                size: 1000,
                legal_description: "Test property 1".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs1".to_string(),
            },
            PropertyMetadata {
                location: "Property 2".to_string(),
                size: 1500,
                legal_description: "Test property 2".to_string(),
                valuation: 150000,
                documents_url: "https://example.com/docs2".to_string(),
            },
            PropertyMetadata {
                location: "Property 3".to_string(),
                size: 2000,
                legal_description: "Test property 3".to_string(),
                valuation: 200000,
                documents_url: "https://example.com/docs3".to_string(),
            },
        ];
        
        let property_ids = contract.batch_register_properties(properties).expect("Failed to batch register");
        assert_eq!(property_ids.len(), 3);
        assert_eq!(property_ids, vec![1, 2, 3]);
        assert_eq!(contract.property_count(), 3);
        
        // Verify all properties were registered correctly
        for (i, &property_id) in property_ids.iter().enumerate() {
            let property = contract.get_property(property_id).unwrap();
            assert_eq!(property.owner, accounts.alice);
            assert_eq!(property.id, property_id);
            assert_eq!(property.metadata.location, format!("Property {}", i + 1));
        }
        
        // Verify owner has all properties
        let owner_properties = contract.get_owner_properties(accounts.alice);
        assert_eq!(owner_properties.len(), 3);
        assert!(owner_properties.contains(&1));
        assert!(owner_properties.contains(&2));
        assert!(owner_properties.contains(&3));
    }

    #[ink::test]
    fn batch_transfer_properties_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register multiple properties
        let properties = vec![
            PropertyMetadata {
                location: "Property 1".to_string(),
                size: 1000,
                legal_description: "Test property 1".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs1".to_string(),
            },
            PropertyMetadata {
                location: "Property 2".to_string(),
                size: 1500,
                legal_description: "Test property 2".to_string(),
                valuation: 150000,
                documents_url: "https://example.com/docs2".to_string(),
            },
        ];
        
        let property_ids = contract.batch_register_properties(properties).expect("Failed to batch register");
        
        // Transfer all properties to Bob
        assert!(contract.batch_transfer_properties(property_ids.clone(), accounts.bob).is_ok());
        
        // Verify all properties were transferred
        for &property_id in &property_ids {
            let property = contract.get_property(property_id).unwrap();
            assert_eq!(property.owner, accounts.bob);
        }
        
        // Verify Alice has no properties
        let alice_properties = contract.get_owner_properties(accounts.alice);
        assert!(alice_properties.is_empty());
        
        // Verify Bob has all properties
        let bob_properties = contract.get_owner_properties(accounts.bob);
        assert_eq!(bob_properties.len(), 2);
        assert!(bob_properties.contains(&1));
        assert!(bob_properties.contains(&2));
    }

    #[ink::test]
    fn batch_update_metadata_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register multiple properties
        let properties = vec![
            PropertyMetadata {
                location: "Property 1".to_string(),
                size: 1000,
                legal_description: "Test property 1".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs1".to_string(),
            },
            PropertyMetadata {
                location: "Property 2".to_string(),
                size: 1500,
                legal_description: "Test property 2".to_string(),
                valuation: 150000,
                documents_url: "https://example.com/docs2".to_string(),
            },
        ];
        
        let property_ids = contract.batch_register_properties(properties).expect("Failed to batch register");
        
        // Update metadata for all properties
        let updates = vec![
            (property_ids[0], PropertyMetadata {
                location: "Updated Property 1".to_string(),
                size: 1200,
                legal_description: "Updated test property 1".to_string(),
                valuation: 120000,
                documents_url: "https://example.com/docs1_updated".to_string(),
            }),
            (property_ids[1], PropertyMetadata {
                location: "Updated Property 2".to_string(),
                size: 1700,
                legal_description: "Updated test property 2".to_string(),
                valuation: 170000,
                documents_url: "https://example.com/docs2_updated".to_string(),
            }),
        ];
        
        assert!(contract.batch_update_metadata(updates).is_ok());
        
        // Verify updates
        let property1 = contract.get_property(property_ids[0]).unwrap();
        assert_eq!(property1.metadata.location, "Updated Property 1");
        assert_eq!(property1.metadata.size, 1200);
        assert_eq!(property1.metadata.valuation, 120000);
        
        let property2 = contract.get_property(property_ids[1]).unwrap();
        assert_eq!(property2.metadata.location, "Updated Property 2");
        assert_eq!(property2.metadata.size, 1700);
        assert_eq!(property2.metadata.valuation, 170000);
    }

    #[ink::test]
    fn batch_transfer_properties_to_multiple_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register multiple properties
        let properties = vec![
            PropertyMetadata {
                location: "Property 1".to_string(),
                size: 1000,
                legal_description: "Test property 1".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs1".to_string(),
            },
            PropertyMetadata {
                location: "Property 2".to_string(),
                size: 1500,
                legal_description: "Test property 2".to_string(),
                valuation: 150000,
                documents_url: "https://example.com/docs2".to_string(),
            },
            PropertyMetadata {
                location: "Property 3".to_string(),
                size: 2000,
                legal_description: "Test property 3".to_string(),
                valuation: 200000,
                documents_url: "https://example.com/docs3".to_string(),
            },
        ];
        
        let property_ids = contract.batch_register_properties(properties).expect("Failed to batch register");
        
        // Transfer properties to different recipients
        let transfers = vec![
            (property_ids[0], accounts.bob),
            (property_ids[1], accounts.charlie),
            (property_ids[2], accounts.django),
        ];
        
        assert!(contract.batch_transfer_properties_to_multiple(transfers).is_ok());
        
        // Verify transfers
        let property1 = contract.get_property(property_ids[0]).unwrap();
        assert_eq!(property1.owner, accounts.bob);
        
        let property2 = contract.get_property(property_ids[1]).unwrap();
        assert_eq!(property2.owner, accounts.charlie);
        
        let property3 = contract.get_property(property_ids[2]).unwrap();
        assert_eq!(property3.owner, accounts.django);
        
        // Verify Alice has no properties
        let alice_properties = contract.get_owner_properties(accounts.alice);
        assert!(alice_properties.is_empty());
    }

    // Portfolio Management Tests
    
    #[ink::test]
    fn get_portfolio_summary_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register multiple properties
        let properties = vec![
            PropertyMetadata {
                location: "Property 1".to_string(),
                size: 1000,
                legal_description: "Test property 1".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs1".to_string(),
            },
            PropertyMetadata {
                location: "Property 2".to_string(),
                size: 1500,
                legal_description: "Test property 2".to_string(),
                valuation: 150000,
                documents_url: "https://example.com/docs2".to_string(),
            },
        ];
        
        contract.batch_register_properties(properties).expect("Failed to batch register");
        
        // Get portfolio summary
        let summary = contract.get_portfolio_summary(accounts.alice);
        assert_eq!(summary.property_count, 2);
        assert_eq!(summary.total_valuation, 250000);
        assert_eq!(summary.average_valuation, 125000);
        assert_eq!(summary.total_size, 2500);
        assert_eq!(summary.average_size, 1250);
    }

    #[ink::test]
    fn get_portfolio_details_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register multiple properties
        let properties = vec![
            PropertyMetadata {
                location: "Property 1".to_string(),
                size: 1000,
                legal_description: "Test property 1".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs1".to_string(),
            },
            PropertyMetadata {
                location: "Property 2".to_string(),
                size: 1500,
                legal_description: "Test property 2".to_string(),
                valuation: 150000,
                documents_url: "https://example.com/docs2".to_string(),
            },
        ];
        
        let property_ids = contract.batch_register_properties(properties).expect("Failed to batch register");
        
        // Get portfolio details
        let details = contract.get_portfolio_details(accounts.alice);
        assert_eq!(details.owner, accounts.alice);
        assert_eq!(details.total_count, 2);
        assert_eq!(details.properties.len(), 2);
        
        // Verify property details
        let prop1 = &details.properties[0];
        assert_eq!(prop1.id, property_ids[0]);
        assert_eq!(prop1.location, "Property 1");
        assert_eq!(prop1.size, 1000);
        assert_eq!(prop1.valuation, 100000);
        
        let prop2 = &details.properties[1];
        assert_eq!(prop2.id, property_ids[1]);
        assert_eq!(prop2.location, "Property 2");
        assert_eq!(prop2.size, 1500);
        assert_eq!(prop2.valuation, 150000);
    }

    // Analytics Tests
    
    #[ink::test]
    fn get_global_analytics_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register properties for Alice
        let alice_properties = vec![
            PropertyMetadata {
                location: "Alice Property 1".to_string(),
                size: 1000,
                legal_description: "Test property".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs".to_string(),
            },
        ];
        contract.batch_register_properties(alice_properties).expect("Failed to register Alice properties");
        
        // Register properties for Bob
        set_caller(accounts.bob);
        let bob_properties = vec![
            PropertyMetadata {
                location: "Bob Property 1".to_string(),
                size: 1500,
                legal_description: "Test property".to_string(),
                valuation: 150000,
                documents_url: "https://example.com/docs".to_string(),
            },
            PropertyMetadata {
                location: "Bob Property 2".to_string(),
                size: 2000,
                legal_description: "Test property".to_string(),
                valuation: 200000,
                documents_url: "https://example.com/docs".to_string(),
            },
        ];
        contract.batch_register_properties(bob_properties).expect("Failed to register Bob properties");
        
        // Get global analytics
        let analytics = contract.get_global_analytics();
        assert_eq!(analytics.total_properties, 3);
        assert_eq!(analytics.total_valuation, 450000);
        assert_eq!(analytics.average_valuation, 150000);
        assert_eq!(analytics.total_size, 4500);
        assert_eq!(analytics.average_size, 1500);
        assert_eq!(analytics.unique_owners, 2);
    }

    #[ink::test]
    fn get_properties_by_price_range_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register properties with different valuations
        let properties = vec![
            PropertyMetadata {
                location: "Cheap Property".to_string(),
                size: 1000,
                legal_description: "Test property".to_string(),
                valuation: 50000,
                documents_url: "https://example.com/docs".to_string(),
            },
            PropertyMetadata {
                location: "Medium Property".to_string(),
                size: 1500,
                legal_description: "Test property".to_string(),
                valuation: 150000,
                documents_url: "https://example.com/docs".to_string(),
            },
            PropertyMetadata {
                location: "Expensive Property".to_string(),
                size: 2000,
                legal_description: "Test property".to_string(),
                valuation: 250000,
                documents_url: "https://example.com/docs".to_string(),
            },
        ];
        
        contract.batch_register_properties(properties).expect("Failed to batch register");
        
        // Get properties in medium price range
        let medium_properties = contract.get_properties_by_price_range(100000, 200000);
        assert_eq!(medium_properties.len(), 1);
        assert_eq!(medium_properties[0], 2); // Medium Property
        
        // Get properties in high price range
        let high_properties = contract.get_properties_by_price_range(200000, 300000);
        assert_eq!(high_properties.len(), 1);
        assert_eq!(high_properties[0], 3); // Expensive Property
        
        // Get all properties
        let all_properties = contract.get_properties_by_price_range(0, 300000);
        assert_eq!(all_properties.len(), 3);
        assert!(all_properties.contains(&1));
        assert!(all_properties.contains(&2));
        assert!(all_properties.contains(&3));
    }

    #[ink::test]
    fn get_properties_by_size_range_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register properties with different sizes
        let properties = vec![
            PropertyMetadata {
                location: "Small Property".to_string(),
                size: 500,
                legal_description: "Test property".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs".to_string(),
            },
            PropertyMetadata {
                location: "Medium Property".to_string(),
                size: 1500,
                legal_description: "Test property".to_string(),
                valuation: 150000,
                documents_url: "https://example.com/docs".to_string(),
            },
            PropertyMetadata {
                location: "Large Property".to_string(),
                size: 2500,
                legal_description: "Test property".to_string(),
                valuation: 200000,
                documents_url: "https://example.com/docs".to_string(),
            },
        ];
        
        contract.batch_register_properties(properties).expect("Failed to batch register");
        
        // Get properties in medium size range
        let medium_properties = contract.get_properties_by_size_range(1000, 2000);
        assert_eq!(medium_properties.len(), 1);
        assert_eq!(medium_properties[0], 2); // Medium Property
        
        // Get properties in large size range
        let large_properties = contract.get_properties_by_size_range(2000, 3000);
        assert_eq!(large_properties.len(), 1);
        assert_eq!(large_properties[0], 3); // Large Property
        
        // Get all properties
        let all_properties = contract.get_properties_by_size_range(0, 3000);
        assert_eq!(all_properties.len(), 3);
        assert!(all_properties.contains(&1));
        assert!(all_properties.contains(&2));
        assert!(all_properties.contains(&3));
    }

    // Gas Monitoring Tests
    
    #[ink::test]
    fn gas_metrics_tracking_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Perform some operations
        let metadata = PropertyMetadata {
            location: "Test Property".to_string(),
            size: 1000,
            legal_description: "Test property".to_string(),
            valuation: 100000,
            documents_url: "https://example.com/docs".to_string(),
        };
        
        contract.register_property(metadata).expect("Failed to register");
        
        // Get gas metrics
        let metrics = contract.get_gas_metrics();
        assert_eq!(metrics.total_operations, 1);
        assert_eq!(metrics.last_operation_gas, 10000);
        assert_eq!(metrics.average_operation_gas, 10000);
        assert_eq!(metrics.min_gas_used, 10000);
        assert_eq!(metrics.max_gas_used, 10000);
    }

    #[ink::test]
    fn performance_recommendations_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Perform multiple operations to generate recommendations
        let metadata = PropertyMetadata {
            location: "Test Property".to_string(),
            size: 1000,
            legal_description: "Test property".to_string(),
            valuation: 100000,
            documents_url: "https://example.com/docs".to_string(),
        };
        
        // Register multiple properties
        for _ in 0..5 {
            contract.register_property(metadata.clone()).expect("Failed to register");
        }
        
        // Get performance recommendations
        let recommendations = contract.get_performance_recommendations();
        assert!(!recommendations.is_empty());
        
        // Should contain general recommendations
        let recommendation_strings: Vec<&str> = recommendations.iter().map(|s| s.as_str()).collect();
        assert!(recommendation_strings.contains(&"Use batch operations for multiple property transfers"));
        assert!(recommendation_strings.contains(&"Prefer portfolio analytics over individual property queries"));
        assert!(recommendation_strings.contains(&"Consider off-chain indexing for complex analytics"));
    }

    // Error Cases Tests
    
    #[ink::test]
    fn batch_transfer_unauthorized_fails() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register properties
        let properties = vec![
            PropertyMetadata {
                location: "Property 1".to_string(),
                size: 1000,
                legal_description: "Test property".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs".to_string(),
            },
        ];
        
        let property_ids = contract.batch_register_properties(properties).expect("Failed to batch register");
        
        // Try to transfer as unauthorized user
        set_caller(accounts.bob);
        assert_eq!(contract.batch_transfer_properties(property_ids, accounts.charlie), Err(Error::Unauthorized));
    }

    #[ink::test]
    fn batch_update_metadata_unauthorized_fails() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Register properties
        let properties = vec![
            PropertyMetadata {
                location: "Property 1".to_string(),
                size: 1000,
                legal_description: "Test property".to_string(),
                valuation: 100000,
                documents_url: "https://example.com/docs".to_string(),
            },
        ];
        
        let property_ids = contract.batch_register_properties(properties).expect("Failed to batch register");
        
        // Try to update as unauthorized user
        set_caller(accounts.bob);
        let updates = vec![
            (property_ids[0], PropertyMetadata {
                location: "Updated Property".to_string(),
                size: 1200,
                legal_description: "Updated test property".to_string(),
                valuation: 120000,
                documents_url: "https://example.com/docs_updated".to_string(),
            }),
        ];
        
        assert_eq!(contract.batch_update_metadata(updates), Err(Error::Unauthorized));
    }

    #[ink::test]
    fn batch_operations_with_empty_input_works() {
        let accounts = default_accounts();
        set_caller(accounts.alice);
        let mut contract = PropertyRegistry::new();
        
        // Test empty batch register
        let empty_properties: Vec<PropertyMetadata> = vec![];
        let result = contract.batch_register_properties(empty_properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
        
        // Test empty batch transfer
        let empty_transfers: Vec<u64> = vec![];
        assert!(contract.batch_transfer_properties(empty_transfers, accounts.bob).is_ok());
        
        // Test empty batch update
        let empty_updates: Vec<(u64, PropertyMetadata)> = vec![];
        assert!(contract.batch_update_metadata(empty_updates).is_ok());
        
        // Test empty batch transfer to multiple
        let empty_multiple_transfers: Vec<(u64, AccountId)> = vec![];
        assert!(contract.batch_transfer_properties_to_multiple(empty_multiple_transfers).is_ok());
    }
}

