Коли користувач залишає коментар або голос, і він не авторизований, або його обліковий запис не підтверджено, він отримає електронний лист із проханням підтвердити цю дію.

Однак ми робимо все можливе, щоб не надсилати вашим користувачам займі листи, і не надсилаємо більше одного листа для підтвердження за сесію. Дивіться розділ Sessions для отримання додаткової інформації.

За замовчуванням листи підтвердження коментарів виглядають так:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Тіло листа підтвердження за замовчуванням із цитуванням коментаря Александра та кнопкою для підтвердження публікації'; title='Лист підтвердження коментаря' app-screenshot-end]

За замовчуванням листи підтвердження голосів виглядають так:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Лист за замовчуванням із проханням до Девона підтвердити голос, що показує прокоментований коментар та кнопку підтвердження'; title='Лист підтвердження голосу' app-screenshot-end]

За замовчуванням FastComments показує свій логотип і назву у нижньому колонтитулі цих листів:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Низ листа підтвердження із логотипом і назвою FastComments за замовчуванням у нижньому колонтитулі'; title='Нижній колонтитул листа' app-screenshot-end]

Якщо ви використовуєте тарифи Flex або Pro, [ім\'я відправника, електронна пошта та брендинг можна налаштувати](/guide-multiple-sites.html#from-name-email-logo).