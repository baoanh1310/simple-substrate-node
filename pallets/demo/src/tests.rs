use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_student(Origin::signed(1), b"icebear".to_vec(), 22));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::student_id(), 1);
	});
}

#[test]
fn correct_error_for_too_young_student() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::create_student(Origin::signed(1), b"icebear".to_vec(), 15), Error::<Test>::TooYoung);
	});
}
