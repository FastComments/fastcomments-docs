#### Sakai

Sakai підтримує LTI 1.3 Dynamic Registration у випусках з LTI Advantage. З Administration Workspace:

1. Увійдіть як адміністратор Sakai і відкрийте **Administration Workspace**.
2. Виберіть **External Tools** > **Install LTI 1.3 Tool**.
3. Вставте URL реєстрації FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">отримайте його тут</a>) і надішліть.
4. Підтвердьте інструмент, коли рукостискання завершиться.

Після цього інструмент з'явиться в розділі **External Tools**, і його зможуть додавати на сайти їхні адміністратори.

#### Schoology

Інстанси Schoology Enterprise підтримують LTI 1.3, але доступність Dynamic Registration залежить від розгортання. Уточніть це у свого менеджера акаунта Schoology.

Якщо Dynamic Registration недоступний у вашому інстансі Schoology, вам потрібно буде налаштувати інтеграцію вручну, використовуючи такі ендпоінти:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Після того як Schoology надасть вам Client ID і Deployment ID, зверніться до служби підтримки FastComments, щоб зареєструвати конфігурацію у вашому тенанті.

#### Інші платформи LTI 1.3

Будь-яка LMS, що відповідає специфікації IMS LTI 1.3 Advantage, має працювати з тим самим URL реєстрації (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">отримайте його тут</a>). Шукайте налаштування з назвою "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" або подібним.

Якщо ваша платформа підтримує лише ручне налаштування LTI 1.3, використайте чотири ендпоінти, наведені в розділі про Schoology вище, і зверніться до служби підтримки для завершення налаштування.