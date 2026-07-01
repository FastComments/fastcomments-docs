---
הוסף את ה‑SDK של FastComments לקובץ `build.gradle.kts` של האפליקציה שלך:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

ודא שיש לך את מאגר Repsy בקובץ `settings.gradle.kts` של הפרויקט שלך:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // מאגרים אחרים...
    }
}
```
---