---
По подразбиране FastComments ще визуализира връзки по следния начин: [https://exmaple.com](https://exmaple.com) - където URL‑тът на връзката се превръща в кликваем  
HTML елемент <a>.

Някои сайтове може да искат да изключат това, например за да възпрепятстват измамници. Предоставяме това чрез задаване на `Comment HTML Rendering Option` на `Links as Text`.

Това може да се персонализира без код, на страницата за персонализиране на уиджета, за цял домейн или страница:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option']; selector = '.comment-html-rendering-mode'; alt='Опция за рендериране на HTML в коментари зададена на „Links as Text“ в раздел „Разширени опции“ при персонализиране на уиджета'; title='Визуализиране на връзки като текст' app-screenshot-end]