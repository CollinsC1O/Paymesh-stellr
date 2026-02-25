use crate::base::types::GroupMember;
use crate::test_utils::{create_test_group, setup_test_env};
use crate::AutoShareContractClient;
use soroban_sdk::{testutils::Address as _, Address, Vec};

#[test]
#[should_panic(expected = "MaxMembersExceeded")]
fn test_update_members_exceeds_max_members() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap().clone();
    let token = test_env.mock_tokens.get(0).unwrap().clone();

    // Create group with 1 member
    let member1 = Address::generate(&test_env.env);
    let mut members = Vec::new(&test_env.env);
    members.push_back(GroupMember {
        address: member1.clone(),
        percentage: 100,
    });

    let group_id = create_test_group(
        &test_env.env,
        &test_env.autoshare_contract,
        &creator,
        &members,
        10,
        &token,
    );

    // Try to update with 51 members (exceeds MAX_MEMBERS of 50)
    let mut new_members = Vec::new(&test_env.env);
    for _ in 0..51 {
        let member = Address::generate(&test_env.env);
        new_members.push_back(GroupMember {
            address: member,
            percentage: 1,
        });
    }

    // This should fail with MaxMembersExceeded
    client.update_members(&group_id, &creator, &new_members);
}

#[test]
fn test_max_members_boundary_success() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap().clone();
    let token = test_env.mock_tokens.get(0).unwrap().clone();

    // Create group with 1 member
    let member1 = Address::generate(&test_env.env);
    let mut members = Vec::new(&test_env.env);
    members.push_back(GroupMember {
        address: member1.clone(),
        percentage: 100,
    });

    let group_id = create_test_group(
        &test_env.env,
        &test_env.autoshare_contract,
        &creator,
        &members,
        10,
        &token,
    );

    // Update with exactly 50 members (MAX_MEMBERS) - should succeed
    let mut new_members = Vec::new(&test_env.env);
    for _ in 0..50 {
        let member = Address::generate(&test_env.env);
        new_members.push_back(GroupMember {
            address: member,
            percentage: 2,
        });
    }

    // This should succeed
    client.update_members(&group_id, &creator, &new_members);

    // Verify all members were added
    let group_members = client.get_group_members(&group_id);
    assert_eq!(group_members.len(), 50);
}
