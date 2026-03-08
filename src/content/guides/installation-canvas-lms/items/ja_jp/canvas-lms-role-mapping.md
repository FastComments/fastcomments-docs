CanvasのロールはLTI起動時に自動的にFastCommentsのロールにマッピングされます。手動の設定は不要です。

#### ロールのマッピング

| Canvasロール | FastCommentsロール | 権限 |
|---|---|---|
| **Administrator** | Admin | アカウントへの完全なアクセス、すべてのコメントと設定の管理 |
| **Instructor** | Moderator | コメントの編集・削除、スレッドのピン留め、ディスカッションの管理 |
| **Learner** | Commenter | コメントの投稿、返信、投票、メンションの使用 |

#### 動作の仕組み

ユーザーがCanvasからFastCommentsを起動すると、LTI 1.3プロトコルにそのユーザーのCanvasロールが含まれます。FastCommentsはこのロールを読み取り、適切な権限を自動的に割り当てます。

ユーザーが複数のロールを持っている場合（例: InstructorでありAdminでもある場合）、最も高い権限のロールが適用されます。

---