#### "Токен за регистрацију није пронађен, истекао је или је већ искоришћен"

Токен у вашем registration URL-у (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">преузмите га овде</a>) важи 30 минута и може се користити само једном. Ако је ваш LMS потрошио више времена, или ако је регистрација покушана поново након успешне регистрације, токен ће бити одбачен. Генеришите нови URL на страници за конфигурацију FastComments LTI 1.3 и почните испочетка.

#### "Платформа је одбила регистрацију"

Ваш LMS је одбио процес размене регистрације. Најчешћи узроци:

- **Алат је већ регистрован под истим именом клијента.** Неке платформе (нарочито D2L) одбијају другу регистрацију "FastComments" док претходна не буде обрисана. Уклonite стари алат у вашем LMS-у, па покушајте поново.
- **Погрешно поље у LMS-у.** Уверите се да сте налепили URL у поље **registration / tool initiation registration endpoint**, а не у поље за launch URL или login URL.
- **LMS заправо не подржава Dynamic Registration.** Старије верзије Moodle и Blackboard најављују LTI 1.3 али дозвољавају само ручну конфигурацију. Проверите документацију ваше платформе.

#### "Неуспешно преузимање конфигурације платформе"

FastComments није могао да прочита openid-configuration документ вашег LMS-а. Ово је ретко и обично значи да је LMS послао неисправан или недоступан discovery URL. Контактирајте подршку вашег LMS-а.

#### Launch shows "Configuration not found"

Или је конфигурација у FastComments обрисана, или је покретање дошло од `iss`/`client_id` пара који не препознајемо. Ако сте обрисали и поново регистровали, упутите ваш LMS да уклони и поново дода алат FastComments како би добио нови client_id.

#### Launch shows "Deployment not registered"

Покренули сте FastComments из Brightspace/Moodle/Blackboard deployment-а другачијег од оног у коме је први пут покренут. FastComments закључава `deployment_id` при првом покретању као безбедносну проверу. Да бисте додали нови deployment под истим клијентом, контактирајте подршку - ми ћемо додати deployment ID у конфигурацију.

#### Launch shows "Unsupported message_type"

LMS је послао LTI поруку коју FastComments не обрађује (нпр. `LtiSubmissionReviewRequest`). FastComments подржава само стандардне resource-link launch и deep-linking токове. Обратите нам се ако вам је потребно да додамо одређени тип поруке.

#### Iframe doesn't resize

Већина LMS-ова аутоматски мења величину LTI iframe-ова. Ако ваш не ради то, проверите да ли поставке покретања LMS-а дозвољавају алату да шаље postMessage догађаје родитељском фрејму. FastComments емитује и Canvas-style (`lti.frameResize`) и IMS-spec (`org.imsglobal.lti.frameResize`) resize поруке.