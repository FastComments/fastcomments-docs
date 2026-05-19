#### Πώς εμφανίζονται τα σχόλια στα μαθήματά σας

Μόλις η ενσωμάτωση LTI ενεργοποιηθεί και η External App εγκατασταθεί, το FastComments λειτουργεί αυτόματα με βάση τα placements που έχετε ρυθμίσει:

#### Assignment View

If the **Assignment View** placement is enabled, comments appear automatically below every assignment in the course. Students and instructors see a threaded comment section when they view an assignment — no extra setup is needed per assignment.

Each assignment gets its own separate comment thread.

#### Rich Content Editor Button

If the **Editor Button** placement is enabled, instructors can embed FastComments into any content that uses the Rich Content Editor:

1. Edit a **Page**, **Quiz**, or **Announcement**.
2. In the Rich Content Editor toolbar, click the **FastComments** button.
3. FastComments is automatically embedded into the content.
4. Save the page.

When students view the page, the embedded FastComments widget loads with a comment thread unique to that page.

#### Αυτόματο SSO

In both placements, students are signed in via their Canvas account automatically. Names, emails, and avatars are synced through the LTI launch, no separate login is needed.

#### Lock Down Public Access (Recommended)

By default, FastComments comment data is publicly readable. Anyone who can guess a thread's URL or API endpoint can view its comments, even outside Canvas. For course discussions you almost certainly want to restrict viewing to enrolled students only.

Open your <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">σελίδα προσαρμογής του widget</a> and create a rule with **Require SSO To View Comments** enabled, then set the security level to **Secure SSO** so threads can only be loaded through the signed LTI launch.

See [Προστασία νημάτων σχολίων με Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for the full walkthrough, including how to scope the rule to a single domain or page.