Интеграция FastComments LTI 1.3 следует принципу наименьших привилегий: она использует только те претензии (claims) запуска, которые необходимы для идентификации пользователя, привязки комментариев к правильному курсу и ресурсу, а также применения разрешений на основе ролей.

Остальная часть этой страницы сопоставляет каждую претензию, которую потребляет интеграция, каждую службу LTI Advantage, к которой она не запрашивает доступ, и каждую категорию данных, которые она не собирает. Рецензенты по безопасности и закупкам могут напрямую брать ответы из таблиц ниже.

## Элементы данных, получаемые от LMS

Каждый LTI 1.3 запуск содержит подписанный JWT от LMS. FastComments извлекает следующие претензии из этого JWT и не использует ничего другого:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Identifies the user consistently across launches so the same person resolves to the same FastComments SSO user | Yes | Yes, as part of a stable internal SSO ID |
| Display name | `name` | Attribution shown next to the user's comments | Yes (falls back to "LMS User" if absent) | Yes |
| Email | `email` | Account matching, notifications, moderation, support correspondence | Optional (the integration works without it) | Yes when provided |
| Avatar URL | `picture` | Displayed on the user's comments | Optional | URL only; FastComments does not download or rehost the image |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Determines whether the user is administrator, instructor (moderator), or learner | Yes | Derived `isAdmin` / `isModerator` flags on the SSO session |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Associates the comment thread with the correct LMS course | Yes | Yes, as part of the resolved page identifier |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Associates comments with the correct activity or tool placement inside the course | Yes when present | Yes, as part of the resolved page identifier |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Routes the launch to the correct FastComments tenant configuration | Yes | Yes, on the FastComments LTI configuration record |

## Претензии и области (scopes), заявленные при регистрации

Во время динамической регистрации LTI 1.3 FastComments регистрирует себя с `scope: ""` (без дополнительных OAuth-областей) и заявляет только следующие претензии OpenID Connect:

`iss`, `sub`, `name`, `email`, `picture`

Она регистрирует два типа сообщений:

- `LtiResourceLinkRequest` - стандартный запуск курса в FastComments.
- `LtiDeepLinkingRequest` - позволяет преподавателям разместить инструмент FastComments внутри курса.

Дополнительные токены доступа от LMS не запрашиваются.

## Службы LTI Advantage, к которым не запрашивается доступ

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | The integration does not need a course roster; user identity arrives with each launch |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | No | The integration is not gradebook-aware |
| Deep Linking beyond the standard placement return | No additional data | Deep linking is used only for instructor placement of the tool; no course content is enumerated |

## Данные, которые не собираются

Помимо самого LTI, FastComments не запрашивает и не получает от LMS или пользователя следующее:

| Category | Collected? |
|----------|------------|
| Student grades | No |
| Assignment submissions | No |
| Attendance records | No |
| Full course rosters | No |
| Government identifiers | No |
| Date of birth | No |
| Postal address or phone number | No |
| Financial information | No |
| LMS administrator credentials | No |

## Границы доступа

- FastComments получает данные только внутри авторизованного запуска LTI 1.3, подписанного зарегистрированными ключами LMS. Интеграция не делает обратных запросов в LMS за дополнительной информацией.
- Токены запуска одноразовые и с коротким сроком действия. Повторно проигранные или истекшие токены отклоняются.
- Администраторы LMS контролируют, где инструмент развернут внутри их платформы. Например, D2L Brightspace поддерживает ограничение по org-unit для каждого развертывания и настройки безопасности на уровне развертывания, что позволяет администраторам ограничивать инструмент определёнными курсами или организационными единицами, а не делать его доступным глобально. Moodle, Blackboard, Sakai и Schoology предлагают эквивалентные средства управления развертываниями в своих реализациях LTI 1.3.

## Хранение и сохранение данных

FastComments хранит данные, полученные через LTI, на протяжении активного периода обслуживания комментариев и в соответствии с настройками срока хранения, настроенными клиентом. Данные комментариев хранятся в продукции, где данные зашифрованы в состоянии покоя. При прекращении учётной записи или по письменному запросу на удаление FastComments удаляет или анонимизирует данные клиента в соответствии с применимым соглашением.

Для полного описания хранения и обработки данных смотрите <a href="https://fastcomments.com/privacy-policy" target="_blank">Политику конфиденциальности FastComments</a>.

## Частота пересмотра

Любая новая функция LTI, для которой потребуются дополнительные претензии, области или службы LTI Advantage, проходит ревью перед выпуском, чтобы подтвердить, что запрашиваемый доступ необходим и соразмерен реализуемой функции.

## Короткое заявление для анкет по безопасности

> FastComments applies least privilege and data minimization to its LTI 1.3 integration. The integration uses only the LTI launch claims required to authenticate the user (`sub`, `name`, `email`, `picture`), determine their role, and identify the course and resource that comments belong to. FastComments does not request Names and Role Provisioning Services, Assignment and Grade Services, gradebook data, attendance, full rosters, or LMS administrative access. LMS administrators retain control over which org units, courses, and deployments the tool is available in.