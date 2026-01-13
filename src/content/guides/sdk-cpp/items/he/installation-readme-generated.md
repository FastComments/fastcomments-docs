### התקנת תלויות

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### בנייה מהמקור

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

### תוכן הספרייה

ספרייה זו מכילה את לקוח ה-API שנוצר וכלי ה-SSO שמקלים על העבודה עם ה-API.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### API ציבורי מול API מאובטח

ללקוח ה-API יש שתי מחלקות, `DefaultAPI` ו-`PublicAPI`. המחלקה `DefaultAPI` מכילה שיטות שדורשות את מפתח ה-API שלך, ו-`PublicAPI` מכילה קריאות API
שניתן לבצע ישירות מהדפדפן/מכשיר נייד/וכו' ללא אימות.