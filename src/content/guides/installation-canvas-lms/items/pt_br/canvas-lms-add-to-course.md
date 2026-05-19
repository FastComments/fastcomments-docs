#### Como os Comentários Aparecem em Seus Cursos

Depois que a integração LTI estiver habilitada e o App Externo instalado, o FastComments funciona automaticamente com base nos placements que você configurou:

#### Assignment View

If the **Assignment View** placement is enabled, comments appear automatically below every assignment in the course. Students and instructors see a threaded comment section when they view an assignment — no extra setup is needed per assignment.

Each assignment gets its own separate comment thread.

#### Rich Content Editor Button

1. Edit a **Page**, **Quiz**, or **Announcement**.
2. In the Rich Content Editor toolbar, click the **FastComments** button.
3. FastComments is automatically embedded into the content.
4. Save the page.

When students view the page, the embedded FastComments widget loads with a comment thread unique to that page.

#### SSO Automático

In both placements, students are signed in via their Canvas account automatically. Names, emails, and avatars are synced through the LTI launch, no separate login is needed.

#### Restringir o Acesso Público (Recomendado)

By default, FastComments comment data is publicly readable. Anyone who can guess a thread's URL or API endpoint can view its comments, even outside Canvas. For course discussions you almost certainly want to restrict viewing to enrolled students only.

Open your <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalização do widget</a> and create a rule with **Require SSO To View Comments** enabled, then set the security level to **Secure SSO** so threads can only be loaded through the signed LTI launch.

See [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for the full walkthrough, including how to scope the rule to a single domain or page.