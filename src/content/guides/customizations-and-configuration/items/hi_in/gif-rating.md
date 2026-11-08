[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

डिफ़ॉल्ट रूप से, FastComments कमेंट विजेट `gif rating` को `pg` पर सेट करेगा।

उपलब्ध विकल्प `g`, `pg`, `pg-13`, और `r` हैं।

इसे कोड में या UI के माध्यम से सेट किया जा सकता है। कोड में हम इसे इस प्रकार कर सकते हैं:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Gif Rating सेट करें'; code-example-end]

UI में, आप इसे `Gif Picker Rating` के अंतर्गत पाएंगे, जब तक कि `Disable Image Uploads?` चेक नहीं किया गया है।

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='विजेट कस्टमाइज़ेशन पेज पर Gif Picker Rating ड्रॉपडाउन जो g, pg, pg-13 और r विकल्प देता है'; title='Gif Rating सेट करना' app-screenshot-end]