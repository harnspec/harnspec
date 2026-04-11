# SDD Examples

Practical examples of how to apply Spec-Driven Development in real-world scenarios.

## Example 1: Feature Implementation (Umbrella Approach)

When building a large feature, break it into smaller pieces.

1. **Create the parent**:
   ```bash
   harnspec create "user-authentication" --title "Unified User Authentication System" --priority high
   ```

2. **Define the umbrella overview**:
   Update `specs/001-user-authentication/README.md` with:
   - Goals: Support Google and Email login.
   - Success Criteria: Both methods work; sessions persist.

3. **Create children**:
   ```bash
   harnspec create "google-oauth" --title "Implement Google OAuth 2.0 Integration" --priority high
   harnspec create "email-login" --title "Implement Magic Link Email Login" --priority medium
   harnspec create "session-management" --title "Persistent Session Handling via Redis" --priority medium
   ```

4. **Establish hierarchy**:
   ```bash
   harnspec rel add 002 --parent 001
   harnspec rel add 003 --parent 001
   harnspec rel add 004 --parent 001
   ```

5. **Set dependencies**:
   ```bash
   # Both login methods depend on session management
   harnspec rel add 002 --depends-on 004
   harnspec rel add 003 --depends-on 004
   ```

6. **Implement**:
   Run `harnspec update 002 --status in-progress` and start coding.

---

## Example 1b: Specifying a Vague Idea (Proposal Mode)

When you don't know how to break down a feature, enter **Proposal Mode** (AI Cognitive Workflow).

1. **Start with an idea**:
   User: "I want offline support for the dashboard"

2. **Follow the AI cognitive phases**:
   - **Brainstorm & Clarify**: The agent asks questions about syncing strategy and conflict resolution.
   - **Seek Confirmation**: The agent proposes a feature breakdown (e.g., IndexedDB storage, sync queue, UI indicators) and waits for user approval.
   - **Confirm**: User reviews the proposed breakdown and says "Yes, looks good."

3. **Generation**:
   The agent automatically creates the spec hierarchy:
   ```bash
   harnspec create "offline-dashboard-support" --title "Offline Dashboard Support" --parent ...
   harnspec create "local-storage-layer" --title "Local Storage Layer" --parent offline-dashboard-support
   harnspec create "background-sync-queue" --title "Background Sync Queue" --parent offline-dashboard-support
   harnspec create "offline-status-ui" --title "Offline Status UI" --parent offline-dashboard-support
   ```

4. **Review the result**:
   ```bash
   # Visual panorama of the generated specs
   harnspec board
   ```

---

## Example 2: Bug Fix (Simple Approach)

For smaller units of work, keep it lean.

1. **Create a quick spec**:
   ```bash
   harnspec create "fix-login-redirect-loop" --title "Fix: Login Redirect Loop on Expiry" --priority critical --tag bug
   ```

2. **Add requirements**:
   - [ ] Verify loop occurs on token expiration.
   - [ ] Update `useAuth` hook to handle redirect.
   - [ ] Add regression test `auth-redirect.test.ts`.

3. **Verify and close**:
   Run `pnpm typecheck && pnpm test`.
   Once pass:
   ```bash
   harnspec update 005 --status complete
   ```

---

## Example 3: Refactoring (Discovery Driven)

When the existing code is messy, use the CLI for discovery.

1. **Check status**:
   ```bash
   harnspec board
   ```

2. **Search for existing refactors**:
   ```bash
   harnspec search "refactor"
   ```

3. **Create a new plan**:
   ```bash
   harnspec create "refactor-api-layer" --title "Standardize API Client Error Handling" --priority medium
   ```

4. **Validate technical approach**:
   Research `src/api/client.ts` and add specific filenames to the spec requirements before starting.

---

## Tips for Better AI Collaboration

- **Be specific** in your spec titles and descriptions.
- **Reference existing code** directly by filename in the requirements.
- **Use the `board` command** after creating a spec to see where it fits.
- **Run `validate`** to catch formatting or structural errors early.
