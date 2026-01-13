### מניעת דליפות זיכרון

כדי למנוע דליפות זיכרון בעת שימוש ב־FastComments views ב־Activities או Fragments, תמיד קראו ל־`cleanup()` כאשר התצוגה כבר לא נדרשת:

#### ב־Activities:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // נקה תצוגות FastComments כדי למנוע דליפות זיכרון
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### ב־Fragments:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // נקה תצוגות FastComments כאשר תצוגת ה־Fragment נהרסת
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // ניקוי נוסף כאשר ה־Fragment נהרס
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### בעת החלפת Fragments:
```java
// לפני החלפה או הסרה של fragment המכיל תצוגות FastComments
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// ואז המשיכו בעסקת ה־fragment
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**חשוב**: תמיד קראו לשיטות `cleanup()` כדי למנוע דליפות זיכרון, במיוחד כאשר:
- ה־Activities נהרסות
- תצוגות ה־Fragment נהרסות
- החלפה בין Fragments
- ניווט החוצה ממסכים המכילים רכיבי FastComments