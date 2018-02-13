// TODO: Should be divided to internal error and spec error
error_chain! {
    errors {
        // Specification errors
        InvalidRequest(desc: String) {
            description("invalid_request")
            display("Invalid request: {}", desc)
        }

        UnauthorizedClient(desc: String) {
            description("unauthorized_client")
            display("Unauthorized client: {}", desc)
        }

        AccessDenied(desc: String) {
            description("access_denied")
            display("Access denied: {}", desc)
        }

        UnsupportedResponseType(desc: String) {
            description("unsupported_response_type")
            display("Unsupported response type: {}", desc)
        }

        InvalidScope(desc: String) {
            description("invalid_scope")
            display("Invalid scope: {}", desc)
        }

        InvalidClient(desc: String) {
            description("invalid_client")
            display("Invalid client: {}", desc)
        }

        InvalidGrant(desc: String) {
            description("invalid_grant")
            display("Invalid grant: {}", desc)
        }

        UnsupportedGrantType(desc: String) {
            description("unsupported_grant_type")
            display("Unsupported grant type: {}", desc)
        }

        ServerError(desc: String) {
            description("server_error")
            display("Server error: {}", desc)
        }

        TemporarilyUnavailable(desc: String) {
            description("temporarily_unavailable")
            display("Temporarily unavailable: {}", desc)
        }

        InvalidToken(desc: String) {
            description("invalid_token")
            display("Invalid token: {}", desc)
        }

        UserinfoError(desc: String) {
            description("invalid_token")
            display("{}", desc)
        }

        // Application errors
        RequireLogin(desc: String) {
            description("login_required")
            display("Login required: {}", desc)
        }

        EntityNotFound(desc: String) {
            description("entity_not_found")
            display("Entity not found: {}", desc)
        }

        LoginFailed(desc: String) {
            description("login_failed")
            display("Login failed: {}", desc)
        }

        DuplicatedEntity(desc: String) {
            description("duplicated_entity")
            display("Duplicated entity: {}", desc)
        }

        ConflictDetected(desc: String) {
            description("conflict_detected")
            display("Conflict detected: {}", desc)
        }

        WrongPassword(desc: String) {
            description("wrong_password")
            display("Wrong password: {}", desc)
        }
    }
}
