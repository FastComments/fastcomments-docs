#### "Регистрациони токен није пронађен, истекао је или је већ искоришћен"

Токен у вашој регистрационој URL адреси важи 30 минута и може се користити само једном. Ако је вашем LMS-у требало дуже од тога, или ако је регистрација покушана поново након што је већ била успешна, токен ће бити одбачен. Генеришите нову URL адресу на страници FastComments LTI 1.3 Configuration и почните испочетка.

#### "Platform rejected registration"

Ваш LMS је одбацио захтев за регистрацију. Најчешћи разлози:

- **Tool already registered with the same client name.** Неке платформе (нарочито D2L) одбацују другу регистрацију "FastComments" док се претходна не обрише. Уклоните стари алат у вашем LMS-у, па покушајте поново.
- **Wrong field in the LMS.** Уверите се да сте налепили URL у поље **registration / tool initiation registration endpoint**, а не у поље launch URL или login URL.
- **The LMS doesn't actually support Dynamic Registration.** Старије верзије Moodle-а и Blackboard-а оглашавају LTI 1.3, али дозвољавају само ручну конфигурацију. Проверите документацију ваше платформе.

#### "Failed to fetch platform configuration"

FastComments није могао да прочита openid-configuration документ вашег LMS-а. Ово је ретко и обично значи да је LMS доставио неисправан или недоступан discovery URL. Контактирајте подршку вашег LMS-а.

#### Launch shows "Configuration not found"

Или је конфигурација у FastComments-у обрисана, или је покретање дошло из `iss`/`client_id` пара који не препознајемо. Ако сте обрисали и поново регистровали, упутите ваш LMS да уклони и поново дода алат FastComments како би добио нови client_id.

#### Launch shows "Deployment not registered"

Покренули сте FastComments из Brightspace/Moodle/Blackboard deployment-а који се разликује од оног у којем је првобитно покренут. FastComments закључава `deployment_id` при првом покретању као безбедносну проверу. Да бисте додали нови deployment под истим клијентом, контактирајте подршку - ми ћемо додати deployment ID у конфигурацију.

#### Launch shows "Unsupported message_type"

LMS је послао LTI поруку коју FastComments не обрађује (нпр. `LtiSubmissionReviewRequest`). FastComments подржава само стандардне resource-link launch и deep-linking токове. Обратите нам се ако вам је потребан додатак специфичне врсте поруке.

#### Iframe doesn't resize

Већина LMS-ова аутоматски мења величину LTI iframe-ова. Ако ваш не ради то, проверите да ли подешавања покретања LMS-а дозвољавају алату да шаље postMessage догађаје родитељском фрејму. FastComments емитује и Canvas-стил (`lti.frameResize`) и IMS-спецификацију (`org.imsglobal.lti.frameResize`) resize поруке.