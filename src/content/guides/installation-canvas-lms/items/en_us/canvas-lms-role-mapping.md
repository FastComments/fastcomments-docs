Canvas roles are automatically mapped to FastComments roles during the LTI launch. No manual configuration is needed.

#### Role Mapping

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Administrator** | Admin | Full account access, manage all comments and settings |
| **Instructor** | Moderator | Edit and delete comments, pin threads, manage discussions |
| **Learner** | Commenter | Post comments, reply, vote, and use mentions |

#### How It Works

When a user launches FastComments from Canvas, the LTI 1.3 protocol includes their Canvas role. FastComments reads this role and assigns the appropriate permissions automatically.

If a user has multiple roles (e.g. an Instructor who is also an Admin), the highest-privilege role is used.

---