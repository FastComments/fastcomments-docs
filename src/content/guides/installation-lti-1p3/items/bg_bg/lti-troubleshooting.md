#### "Регистрационният токен не е намерен, е изтекъл или вече е използван"

Токенът в регистрационния ви URL е валиден за 30 минути и може да се използва само веднъж. Ако вашият LMS е отнел повече време от това, или ако регистрацията е била повторена след успешно завършване, токенът ще бъде отхвърлен. Генерирайте нов URL в страницата за конфигурация на FastComments LTI 1.3 и започнете отначало.

#### "Platform rejected registration"

Вашият LMS отказа регистрационното ръкостискане. Най-честите причини:

- **Tool already registered with the same client name.** Някои платформи (особено D2L) отказват втора регистрация на "FastComments", докато предишната не бъде изтрита. Премахнете стария инструмент от вашия LMS и опитайте отново.
- **Wrong field in the LMS.** Уверете се, че сте поставили URL-а във полето **registration / tool initiation registration endpoint**, а не в полето за launch URL или login URL.
- **The LMS doesn't actually support Dynamic Registration.** По-старите версии на Moodle и Blackboard рекламират LTI 1.3, но позволяват само ръчна конфигурация. Проверете документацията на вашата платформа.

#### "Failed to fetch platform configuration"

FastComments не успя да прочете документа openid-configuration на вашия LMS. Това е рядко и обикновено означава, че LMS е предоставил неправилно форматиран или недостъпен discovery URL. Свържете се с поддръжката на вашия LMS.

#### Launch shows "Configuration not found"

Или конфигурацията във FastComments е изтрита, или стартирането е дошло от `iss`/`client_id` чифт, който не разпознаваме. Ако сте изтрили и повторно регистрирали, инструктирайте вашия LMS да премахне и добави отново инструмента FastComments, за да получи новия client_id.

#### Launch shows "Deployment not registered"

Стартирахте FastComments от Brightspace/Moodle/Blackboard разполагане, различно от това, в което беше стартирано първоначално. FastComments фиксира `deployment_id` при първото стартиране като проверка за сигурност. За да добавите ново разполагане под същия клиент, свържете се с поддръжката - ние ще добавим deployment ID към конфигурацията.

#### Launch shows "Unsupported message_type"

LMS изпрати LTI съобщение, което FastComments не обработва (напр. `LtiSubmissionReviewRequest`). FastComments поддържа само стандартните resource-link launch и deep-linking потоци. Свържете се с нас, ако имате нужда да добавим конкретен тип съобщение.

#### Iframe doesn't resize

Повечето LMS автоматично преоразмеряват LTI iframe-ове. Ако вашият не го прави, проверете дали настройките за стартиране на LMS позволяват на инструмента да изпраща postMessage събития към родителския фрейм. FastComments изпраща както Canvas-style (`lti.frameResize`), така и IMS-spec (`org.imsglobal.lti.frameResize`) resize съобщения.

---