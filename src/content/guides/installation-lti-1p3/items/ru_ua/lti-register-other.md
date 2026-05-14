#### Sakai

Sakai поддерживает LTI 1.3 Dynamic Registration в релизах с LTI Advantage. В рабочей области администратора:

1. Войдите как администратор Sakai и откройте **Рабочую область администратора**.
2. Выберите **Внешние инструменты** > **Установить инструмент LTI 1.3**.
3. Вставьте URL регистрации FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить его здесь</a>) и отправьте.
4. Одобрите инструмент после завершения рукопожатия.

Инструмент затем появится в разделе **Внешние инструменты** и может быть добавлен на сайты их администраторами.

#### Schoology

Инстансы Schoology Enterprise поддерживают LTI 1.3, но доступность Dynamic Registration варьируется в зависимости от развертывания. Свяжитесь с вашим менеджером по работе с аккаунтом Schoology.

Если Dynamic Registration недоступна в вашем экземпляре Schoology, вам потребуется настроить интеграцию вручную, используя эти эндпоинты:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

После того как Schoology предоставит вам Client ID и Deployment ID, свяжитесь со службой поддержки FastComments, чтобы зарегистрировать конфигурацию в вашем тенанте.

#### Other LTI 1.3 Platforms

Любая LMS, соответствующая спецификации IMS LTI 1.3 Advantage, должна работать с тем же URL регистрации (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить его здесь</a>). Ищите настройку с названием "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" или похожую.

Если ваша платформа поддерживает только ручную настройку LTI 1.3, используйте четыре эндпоинта, перечисленные в разделе Schoology выше, и свяжитесь со службой поддержки для завершения.