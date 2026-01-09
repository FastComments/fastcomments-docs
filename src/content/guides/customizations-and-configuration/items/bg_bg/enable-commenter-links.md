[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще поиска от потребителя само техния коментар, потребителско име и имейл.

Въпреки това, в някои ситуации може да искате потребителят да остави връзка към собствения си блог или уебсайт.

Можем да разрешим показването на допълнително поле за въвеждане за URL на уебсайта на потребителя, като зададем флага **enableCommenterLinks** на true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Когато такъв URL бъде предоставен, акаунтът на потребителя ще бъде актуализиран и всички негови потребителски имена във всички минали и бъдещи коментари ще бъдат с връзка към този URL.

Това може да бъде персонализирано без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---