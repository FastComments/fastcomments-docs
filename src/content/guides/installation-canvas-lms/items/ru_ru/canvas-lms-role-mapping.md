Роли Canvas автоматически сопоставляются с ролями FastComments во время запуска LTI. Ручная настройка не требуется.

#### Сопоставление ролей

| Роль Canvas | Роль FastComments | Разрешения |
|---|---|---|
| **Administrator** | Admin | Full account access, manage all comments and settings |
| **Instructor** | Moderator | Edit and delete comments, pin threads, manage discussions |
| **Learner** | Commenter | Post comments, reply, vote, and use mentions |

#### Как это работает

Когда пользователь запускает FastComments из Canvas, протокол LTI 1.3 передаёт его роль Canvas. FastComments считывает эту роль и автоматически назначает соответствующие разрешения.

Если у пользователя несколько ролей (например, an Instructor who is also an Admin), используется роль с наивысшими привилегиями.

---