FastComments があなたの LMS に登録されると、インストラクターは他の LTI 外部ツールを追加するのと同じ方法でコースに追加します。

#### D2L Brightspace

コースのコンテンツ領域で:

1. Click **Add Existing Activities** > **External Learning Tools**.
2. Pick **FastComments** from the list.
3. The tool appears as a topic in the content area. Open it once to initialize the comment thread for that resource.

#### Moodle

コース内で:

1. Turn **Edit mode** on.
2. In the section where you want comments, click **Add an activity or resource**.
3. Choose **FastComments** from the activity chooser.
4. Save. Students see the comment thread embedded in the section.

#### Blackboard Learn

コース内で:

1. Navigate to a Content Area.
2. Click **Build Content** > **FastComments** (under "Learning Tools").
3. Configure a name and submit.

#### Sakai

Site maintainers add the tool through **Site Info** > **Manage Tools** > scroll to **External Tools** > select **FastComments** > **Continue**.

#### How Threads Are Scoped

FastComments creates a separate comment thread per **(LMS instance, course, resource link)**. That means:

- Two different courses in the same LMS get separate threads, even if they use the same tool name.
- The same FastComments tool used in two places within one course creates two threads.
- Two different Brightspace installations under the same FastComments account get distinct threads - the LMS hostname is part of the thread identifier.

#### SSO

Students don't see a login screen. The LMS sends their identity (name, email, avatar, role) to FastComments via the LTI launch, and FastComments signs them in automatically. Their comments are attributed to their LMS account.

Users with the LMS roles **Instructor** or **Administrator** are auto-mapped to FastComments moderator/admin roles for the thread.