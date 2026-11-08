### मेमोरी लीक को रोकना

Activities या Fragments में FastComments views का उपयोग करते समय मेमोरी लीक को रोकने के लिए, जब view की अब आवश्यकता न हो तो हमेशा `cleanup()` कॉल करें:

#### Activities में:
```java
@Override
protected void onDestroy() {
    super.onDestroy();
    // मेमोरी लीक को रोकने के लिए FastComments views को क्लीन अप करें
    if (feedView != null) {
        feedView.cleanup();
    }
    if (commentsView != null) {
        commentsView.cleanup();
    }
}
```

#### Fragments में:
```java
@Override
public void onDestroyView() {
    super.onDestroyView();
    // fragment view नष्ट होने पर FastComments views को क्लीन अप करें
    if (feedView != null) {
        feedView.cleanup();
        feedView = null;
    }
}

@Override
public void onDestroy() {
    super.onDestroy();
    // fragment नष्ट होने पर अतिरिक्त क्लीनअप
    if (feedSDK != null) {
        feedSDK.cleanup();
        feedSDK = null;
    }
}
```

#### Fragments बदलते समय:
```java
// FastComments views वाले fragment को बदलने या हटाने से पहले
Fragment currentFragment = getSupportFragmentManager().findFragmentById(R.id.container);
if (currentFragment instanceof YourFragmentWithFeedView) {
    ((YourFragmentWithFeedView) currentFragment).cleanupFeedView();
}

// फिर fragment transaction के साथ आगे बढ़ें
getSupportFragmentManager().beginTransaction()
    .replace(R.id.container, newFragment)
    .commit();
```

**महत्वपूर्ण**: मेमोरी लीक को रोकने के लिए हमेशा `cleanup()` मेथड्स कॉल करें, विशेष रूप से जब:
- Activities नष्ट हो जाएं
- Fragment views नष्ट हो जाएं
- Fragments के बीच स्विच किया जा रहा हो
- FastComments components वाली screens से दूर नेविगेट किया जा रहा हो