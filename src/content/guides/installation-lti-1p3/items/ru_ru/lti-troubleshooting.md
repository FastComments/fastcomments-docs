#### "Токен регистрации не найден, истёк или уже использован"

Токен в вашем URL для регистрации (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить его здесь</a>) действителен 30 минут и может быть использован только один раз. Если ваш LMS занял больше времени, или если регистрация была повторена после успешного выполнения, токен будет отклонён. Сгенерируйте новый URL на странице конфигурации FastComments LTI 1.3 и начните заново.

#### "Platform rejected registration"

Ваш LMS отказал в рукопожатии регистрации. Наиболее частые причины:

- **Tool already registered with the same client name.** Некоторые платформы (в частности D2L) отклоняют вторую регистрацию "FastComments", пока предыдущая не удалена. Удалите старый инструмент в вашем LMS, затем повторите попытку.
- **Wrong field in the LMS.** Убедитесь, что вы вставили URL в поле **registration / tool initiation registration endpoint**, а не в поле launch URL или login URL.
- **The LMS doesn't actually support Dynamic Registration.** Старые версии Moodle и Blackboard декларируют поддержку LTI 1.3, но позволяют только ручную настройку. Проверьте документацию вашей платформы.

#### "Failed to fetch platform configuration"

FastComments не смог прочитать документ openid-configuration вашего LMS. Это редкость и обычно означает, что LMS предоставил некорректный или недоступный discovery URL. Свяжитесь со службой поддержки вашего LMS.

#### Launch shows "Configuration not found"

Либо конфигурация в FastComments была удалена, либо запуск пришёл с пары `iss`/`client_id`, которую мы не распознаём. Если вы удалили и заново зарегистрировали, попросите ваш LMS удалить и снова добавить инструмент FastComments, чтобы он получил новый client_id.

#### Launch shows "Deployment not registered"

Вы запустили FastComments из развертывания Brightspace/Moodle/Blackboard, отличного от того, в котором он был запущен впервые. FastComments фиксирует `deployment_id` при первом запуске в качестве проверки безопасности. Чтобы добавить новое развертывание под тем же клиентом, свяжитесь со службой поддержки — мы добавим deployment ID в конфигурацию.

#### Launch shows "Unsupported message_type"

LMS отправила LTI-сообщение, которое FastComments не обрабатывает (например, `LtiSubmissionReviewRequest`). FastComments поддерживает только стандартный resource-link launch и потоки deep-linking. Обратитесь к нам, если нужно добавить поддержку конкретного типа сообщения.

#### Iframe doesn't resize

Большинство LMS автоматически подстраивают размер LTI iframe. Если ваш этого не делает, проверьте, позволяют ли настройки запуска LMS инструменту отправлять события postMessage в родительский фрейм. FastComments посылает сообщения изменения размера как в формате Canvas (`lti.frameResize`), так и в спецификации IMS (`org.imsglobal.lti.frameResize`).

---