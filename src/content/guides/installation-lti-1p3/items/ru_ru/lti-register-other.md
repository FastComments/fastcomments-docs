#### Sakai

Sakai поддерживает LTI 1.3 Dynamic Registration в релизах с LTI Advantage. Из Administration Workspace:

1. Войдите как администратор Sakai и откройте **Administration Workspace**.
2. Выберите **External Tools** > **Install LTI 1.3 Tool**.
3. Вставьте URL регистрации FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить его здесь</a>) и отправьте.
4. Подтвердите инструмент, когда завершится рукопожатие.

После этого инструмент появится в разделе **External Tools**, и его смогут добавлять на сайты их администраторы.

#### Schoology

Экземпляры Schoology Enterprise поддерживают LTI 1.3, но доступность Dynamic Registration зависит от развёртывания. Уточните у вашего менеджера аккаунта Schoology.

Если Dynamic Registration недоступен на вашем экземпляре Schoology, вам потребуется настроить интеграцию вручную, используя следующие эндпоинты:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

После того как Schoology предоставит вам Client ID и Deployment ID, свяжитесь со службой поддержки FastComments, чтобы зарегистрировать конфигурацию на вашем тенанте.

#### Другие платформы LTI 1.3

Любая LMS, соответствующая спецификации IMS LTI 1.3 Advantage, должна работать с тем же URL регистрации (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить его здесь</a>). Ищите настройку с названием «Dynamic Registration», «Tool Registration URL», «Tool initiation registration endpoint» или похожим.

Если ваша платформа поддерживает только ручную настройку LTI 1.3, используйте четыре эндпоинта, перечисленных в разделе о Schoology выше, и свяжитесь со службой поддержки для завершения настройки.