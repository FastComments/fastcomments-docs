#### "Токен регистрации не найден, истёк или уже использован"

Токен в вашем URL для регистрации (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">получить его здесь</a>) действителен в течение 30 минут и может быть использован только один раз. Если ваш LMS занял больше времени, или если попытка регистрации была повторно выполнена после успешной регистрации, токен будет отклонён. Сгенерируйте новый URL на странице конфигурации FastComments LTI 1.3 и начните заново.

#### "Platform rejected registration"

Ваш LMS отклонил попытку регистрации. Наиболее частые причины:

- **Tool already registered with the same client name.** Некоторые платформы (в частности D2L) отклоняют повторную регистрацию "FastComments", пока предыдущая запись не будет удалена. Удалите старый инструмент в вашем LMS, затем повторите попытку.
- **Wrong field in the LMS.** Убедитесь, что вы вставили URL в поле **registration / tool initiation registration endpoint**, а не в поле launch URL или login URL.
- **The LMS doesn't actually support Dynamic Registration.** Старые версии Moodle и Blackboard заявляют поддержку LTI 1.3, но позволяют только ручную настройку. Проверьте документацию вашей платформы.

#### "Failed to fetch platform configuration"

FastComments не смог прочитать документ openid-configuration вашего LMS. Такое случается редко и обычно означает, что LMS предоставил некорректный или недоступный discovery URL. Обратитесь в поддержку вашего LMS.

#### Launch shows "Configuration not found"

Либо конфигурация в FastComments была удалена, либо запуск пришёл от пары `iss`/`client_id`, которую мы не распознаём. Если вы удалили и повторно зарегистрировали, попросите ваш LMS удалить и заново добавить инструмент FastComments, чтобы он получил новый `client_id`.

#### Launch shows "Deployment not registered"

Вы запустили FastComments из развертывания Brightspace/Moodle/Blackboard, отличного от того, в котором он был запущен впервые. FastComments закрепляет `deployment_id` при первом запуске в качестве проверки безопасности. Чтобы добавить новое развертывание для того же клиента, обратитесь в поддержку — мы добавим deployment ID в конфигурацию.

#### Launch shows "Unsupported message_type"

LMS отправил LTI-сообщение, которое FastComments не обрабатывает (например, `LtiSubmissionReviewRequest`). FastComments поддерживает только стандартные потоки resource-link launch и deep-linking. Свяжитесь с нами, если вам нужен конкретный тип сообщения.

#### Iframe doesn't resize

Большинство LMS автоматически подстраивают размер LTI iframe. Если ваш этого не делает, проверьте, что настройки запуска LMS позволяют инструменту отправлять события postMessage в родительский фрейм. FastComments отправляет как сообщения в стиле Canvas (`lti.frameResize`), так и по спецификации IMS (`org.imsglobal.lti.frameResize`) для изменения размера.