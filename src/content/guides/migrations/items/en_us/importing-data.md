While FastComments Support can help with migrations, most can be performed and monitored easily without any intervention by support staff.

We natively support importing exports from the following providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)

By navigating [here](https://fastcomments.com/auth/my-account/manage-data/import) you can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitoring Imports

FastComments uses a job processing system for processing imports and exports. Once the system has picked up your job, it will
periodically report the status of the job in the import or export UI.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Note that the status for imports and exports is viewable by all administrators in the account.

If your job fails, it will not automatically be restarted. The import will have to be attempted again. If any import or export fails,
our system administrators are automatically notified. If we identify an issue, we'll reach out to you to see if we can help.

### Re-Running The Import

During some migrations, it is necessary to run the import multiple times. For example, it is common to do a first pass
migration for testing, and then run the import again with the latest data before flipping the switch.

Re-importing the same content **will not create duplicates**.

### Data Security and Expiration

Import files are not accessible via external requests in any way, and import files are deleted from our system as soon as
the import completes.

---