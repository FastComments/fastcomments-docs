### התקנת תלותים

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### בנייה ממקור

```bash
mkdir build
cd build
cmake ..
make
```

### התקנה

```bash
sudo make install
```

### תכולת הספרייה

ספרייה זו מכילה את לקוח ה‑API שנוצר ואת כלי ה‑SSO כדי להקל על העבודה עם ה‑API.

- [תיעוד ספריית לקוח API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### API ציבוריים מול מאובטחים

עבור לקוח ה‑API, קיימות שלוש מחלקות, `DefaultApi`, `PublicApi`, ו‑`ModerationApi`. ה‑`DefaultApi` מכילה שיטות הדורשות את מפתח ה‑API שלך, ו‑`PublicApi` מכילה שיטות שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו׳ ללא אימות. ה‑`ModerationApi` מספקת חבילת רחבה של API מודרציה בזמן אמת ומהירה. כל שיטה של `ModerationApi` מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או באמצעות עוגיית סשן של FastComments.com.

---