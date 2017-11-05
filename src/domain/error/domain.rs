error_chain! {
    errors {
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

        RequireLogin {
            description("require_login")
            display("Login is required.")
        }

        EntryNotFound {
            description("entry_not_found")
            display("The entry was not found. It may have been already deleted.")
        }

        LoginFailed {
            description("login_failed")
            display("Name or password is not correct.")
        }

        DuplicatedEntry {
            description("duplicated_entry")
            display("The entry already exists.")
        }

        UserinfoError(desc: String) {
            description("invalid_token")
            display("{}", desc)
        }
    }
}
