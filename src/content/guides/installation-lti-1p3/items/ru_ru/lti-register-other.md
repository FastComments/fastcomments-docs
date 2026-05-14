#### Sakai

Sakai поддерживает LTI 1.3 Dynamic Registration в выпусках с LTI Advantage. Из **Рабочей области администратора**:

1. Войдите как администратор Sakai и откройте **Рабочую область администратора**.
2. Выберите **Внешние инструменты** > **Установить инструмент LTI 1.3**.
3. Вставьте URL регистрации FastComments и отправьте.
4. Одобрите инструмент после завершения рукопожатия.

Затем инструмент появится в разделе **Внешние инструменты** и его могут добавлять на сайты их администраторы.

#### Schoology

Инстансы Schoology Enterprise поддерживают LTI 1.3, но доступность Dynamic Registration зависит от развертывания. Спросите вашего менеджера по работе с Schoology.

Если Dynamic Registration недоступен в вашем инстансе Schoology, вам нужно будет настроить интеграцию вручную, используя эти конечные точки:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

После того как Schoology предоставит вам Client ID и Deployment ID, свяжитесь со службой поддержки FastComments, чтобы зарегистрировать конфигурацию на вашем tenant.

#### Other LTI 1.3 Platforms

Любая LMS, которая соответствует спецификации IMS LTI 1.3 Advantage, должна работать с тем же URL регистрации. Ищите настройку с названием "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" или похожую.

Если ваша платформа поддерживает только ручную настройку LTI 1.3, используйте четыре конечные точки, перечисленные в разделе Schoology выше, и свяжитесь со службой поддержки для завершения.