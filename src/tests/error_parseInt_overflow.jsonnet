// Test that parseInt throws an error for values beyond MAX_SAFE_INTEGER
// This should fail with: "Not an integer: numeric value outside safe integer range"
std.parseInt('9007199254740992')  // 2^53, beyond MAX_SAFE_INTEGER
