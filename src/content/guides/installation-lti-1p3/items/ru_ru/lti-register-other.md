#### Sakai

Sakai поддерживает динамическую регистрацию LTI 1.3 в релизах с LTI Advantage. Из **Administration Workspace**:

1. Войдите как администратор Sakai и откройте **Administration Workspace**.
2. Выберите **External Tools** > **Install LTI 1.3 Tool**.
3. Вставьте URL регистрации FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить его здесь</a>) и отправьте.
4. Одобрите инструмент после завершения рукопожатия.

Инструмент затем появляется в разделе **External Tools** и может быть добавлен администраторами сайтов.

#### Schoology

Инстансы Schoology Enterprise поддерживают LTI 1.3, но доступность Dynamic Registration варьируется в зависимости от развертывания. Свяжитесь с вашим менеджером аккаунта Schoology.

Если Dynamic Registration недоступна в вашем экземпляре Schoology, вам нужно будет настроить интеграцию вручную, используя эти конечные точки:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

После того как Schoology предоставит вам Client ID и Deployment ID, свяжитесь со службой поддержки FastComments, чтобы зарегистрировать конфигурацию на вашем tenant.

#### Other LTI 1.3 Platforms

Любая LMS, соответствующая спецификации IMS LTI 1.3 Advantage, должна работать с тем же URL регистрации (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить его здесь</a>). Ищите настройку с пометкой "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" или похожую.

Если ваша платформа поддерживает только ручную настройку LTI 1.3, используйте четыре конечные точки, перечисленные в разделе Schoology выше, и свяжитесь со службой поддержки для завершения настройки.