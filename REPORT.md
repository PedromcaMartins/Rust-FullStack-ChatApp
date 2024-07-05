# Business logic

### Main Webpages

- **Create Account**
    - Form fields: username, email, password, confirm password.
    - Validation: check for existing username/email, password strength.
    - Confirm account creation via email (optional, for better security).

- **Login**
    - Form fields: username/email, password.
    - Option to reset password if forgotten.

- **Logout**
    - Invalidate user session or token.
    - Redirect to the login page.

- **Chatrooms**
    - Display a list of available chatrooms.
    - Option to search for chatrooms by name or topic.

### Chatrooms

- **Create Chatroom**
    - Form fields: chatroom name, description, visibility (public/private).
    - Option to invite users to the chatroom (optional).

- **Join Chatroom**
    - Check if the user is already a member.
    - Add the user to the chatroom members list.

- **Leave Chatroom**
    - Remove the user from the chatroom members list.
    - Option to rejoin later if the chatroom is public or by invitation.

### Chatroom

- **Send Message**
    - Text input for messages.
    - Option to send attachments (images, files, etc.).
    - Implement message throttling to prevent spam.

- **View Messages**
    - Display messages in real-time.
    - Load message history on scroll.

- **View Users in Chatroom**
    - Display a list of online and offline users in the chatroom.
    - Option to private message users (optional).

### Admin Panel

- **Ban User**
    - Permanently or temporarily ban users.
    - Specify the reason for the ban.

- **Unban User**
    - Remove bans from users.
    - Log the action for audit purposes.

- **Ban Chatroom**
    - Prevent any activity in the chatroom.
    - Specify the reason for the ban.

- **Unban Chatroom**
    - Restore the chatroom's functionality.
    - Log the action for audit purposes.

- **Delete Chatroom**
    - Permanently remove a chatroom and all associated data.
    - Confirm action with a warning about data loss.

- **Delete User**
    - Permanently remove a user and all associated data.
    - Confirm action with a warning about data loss.

- **View Chatrooms**
    - Display a list of all chatrooms, with options to manage (edit, delete, ban).

- **View Users**
    - Display a list of all users, with options to manage (edit, delete, ban).

- **View Banned Users**
    - Display a list of all banned users, with options to unban.

- **View Banned Chatrooms**
    - Display a list of all banned chatrooms, with options to unban.

- **View Deleted Chatrooms**
    - Display a list of all deleted chatrooms, with options to restore (optional).

- **View Deleted Users**
    - Display a list of all deleted users, with options to restore (optional).

- **View Messages**
    - Search and filter messages across chatrooms.
    - Option to delete inappropriate messages.

### Additional Suggestions

1. **User Profiles**
    - Allow users to create and edit profiles with optional information (bio, profile picture).

2. **Notifications**
    - Implement notifications for new messages, mentions, chatroom invitations, etc.

3. **Privacy Settings**
    - Allow users to control who can send them messages, see their online status, etc.

4. **Moderation Tools**
    - Implement moderation tools such as reporting messages/users, muting users, etc.

5. **Analytics and Reporting**
    - Provide admins with analytics on chatroom activity, user engagement, etc.

6. **Security Measures**
    - Ensure secure password storage and user authentication mechanisms.
