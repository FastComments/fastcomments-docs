#### "Registration token not found, expired, or already used"

Токен у вашем регистрационом URL-у (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">дохватите га овдје</a>) важи 30 минута и може се користити само једном. Ако је ваш LMS трајао дуже од тога, или ако је регистрација покушана поново након успјешне регистрације, токен ће бити одбијен. Генеришите нови URL на страници за конфигурацију FastComments LTI 1.3 и почните испочетка.

#### "Platform rejected registration"

Ваш LMS је одбио руковање регистрацијом. Најчешћи узроци:

- **Tool already registered with the same client name.** Неки платформи (нарочито D2L) одбијају другу регистрацију "FastComments" док претходна није обрисана. Уклоните стари алат у вашем LMS-у, па покушајте поново.
- **Wrong field in the LMS.** Провјерите да ли сте залијепили URL у поље **registration / tool initiation registration endpoint**, а не у поље **launch URL** или **login URL**.
- **The LMS doesn't actually support Dynamic Registration.** Старије верзије Moodle-а и Blackboard-а оглашавају LTI 1.3 али дозвољавају само ручну конфигурацију. Провјерите документaцију ваше платформе.

#### "Failed to fetch platform configuration"

FastComments није могао прочитати openid-configuration документ вашег LMS-а. Ово је ријетко и обично значи да је LMS доставио неисправан или недостижан discovery URL. Контактирајте подршку вашег LMS-а.

#### Launch shows "Configuration not found"

Или је конфигурација у FastComments избрисана, или је покретање дошло од `iss`/`client_id` пара који не препознајемо. Ако сте избрисали и поново регистровали, наредите вашем LMS-у да уклони и поново дода FastComments алат тако да добије нови `client_id`.

#### Launch shows "Deployment not registered"

Покренули сте FastComments из Brightspace/Moodle/Blackboard deployment-а различитог од оног у којем је први пут покренут. FastComments закључава `deployment_id` при првом покретању као безбједносну провјеру. Да бисте додали нови deployment под истим клиентом, контактирајте подршку — ми ћемо додати deployment ID у конфигурацију.

#### Launch shows "Unsupported message_type"

LMS је послао LTI поруку коју FastComments не обрађује (нпр. `LtiSubmissionReviewRequest`). FastComments подржава само стандардни resource-link launch и deep-linking ток. Јавите нам се ако треба да додамо одређени тип поруке.

#### Iframe doesn't resize

Већина LMS-ова аутоматски прилагођава величину LTI iframe-ова. Ако ваш не ради то, провјерите да ли поставке покретања LMS-а дозвољавају алату да шаље postMessage догађаје родитељском оквиру. FastComments емитује и Canvas-style (`lti.frameResize`) и IMS-spec (`org.imsglobal.lti.frameResize`) поруке за промјену величине.