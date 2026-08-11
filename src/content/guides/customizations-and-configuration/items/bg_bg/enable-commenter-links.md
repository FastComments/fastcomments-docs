[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще иска от потребителя само неговия коментар, потребителско име и имейл.

Въпреки това, в някои ситуации може да искате потребителят да остави връзка към своя блог или уебсайт.

Можем да активираме показването на допълнително поле за въвеждане, където потребителят да остави URL на своя уебсайт, като зададем флага **enableCommenterLinks** на true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Активиране на връзки за коментаторите'; code-example-end]

Когато този URL бъде предоставен, акаунтът на потребителя ще бъде актуализиран и всички негови потребителски имена в минали и бъдещи коментари ще водят към този URL.

Това може да се персонализира без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Страница за персонализиране на уиджет с отметка за връзки на коментаторите, за да се добави поле за уебсайт URL във формуляра за коментар'; title='Активиране на връзки за коментаторите' app-screenshot-end]