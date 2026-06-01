הגדר את מזהה הדייר פעם אחת בקובץ `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

לאחר מכן הוסף תג במקום שבו אתה רוצה שהווידג'ט יופיע, בתבנית, בפוסט או בדף:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

זהו. החלף את `demo` במזהה הדייר של FastComments שלך (תמצא אותו תחת
[Settings > API/SSO](https://fastcomments.com/auth/my-account/api)).