בחר אחת משתי הדרכים הסטנדרטיות להוספת רכיב נושא של Hugo.

### אפשרות A: מודול של Hugo (מומלץ)

מתיקיית השורש של האתר שלך:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

ואז הוסף את הייבוא ל-`hugo.toml` שלך:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### אפשרות B: רכיב תבנית (תת-מודול Git)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

לאחר מכן התייחס אליו מתוך `hugo.toml` שלך. רשום אותו לצד התבנית שלך; הערכים המופיעים מאוחר יותר מנצחים, אז שמור את התבנית שלך ראשונה:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```