#### Sakai

Sakai підтримує динамічну реєстрацію LTI 1.3 у випусках з LTI Advantage. З **Administration Workspace**:

1. Увійдіть як адміністратор Sakai і відкрийте **Administration Workspace**.
2. Виберіть **External Tools** > **Install LTI 1.3 Tool**.
3. Вставте URL реєстрації FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">отримайте його тут</a>) і надішліть.
4. Підтвердіть інструмент, коли рукопотискання буде завершено.

Після цього інструмент з’явиться в **External Tools** і його можна додати на сайти їхніми адміністраторами.

#### Schoology

Інсталяції Schoology Enterprise підтримують LTI 1.3, але наявність Dynamic Registration залежить від розгортання. Зверніться до вашого менеджера облікового запису Schoology.

Якщо Dynamic Registration недоступна у вашій інсталяції Schoology, вам потрібно буде налаштувати інтеграцію вручну, використовуючи ці кінцеві точки:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Після того як Schoology надасть вам Client ID і Deployment ID, зв’яжіться зі службою підтримки FastComments, щоб зареєструвати конфігурацію на вашому тенанті.

#### Other LTI 1.3 Platforms

Будь-яка LMS, яка дотримується специфікації IMS LTI 1.3 Advantage, повинна працювати з тим же URL реєстрації (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">отримайте його тут</a>). Шукайте налаштування з позначенням "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" або подібне.

Якщо ваша платформа підтримує лише ручне налаштування LTI 1.3, використовуйте ті самі чотири кінцеві точки, перелічені в розділі Schoology вище, і зв’яжіться зі службою підтримки для завершення налаштування.