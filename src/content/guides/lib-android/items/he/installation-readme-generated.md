הוסף את ה‑FastComments SDK לקובץ `build.gradle.kts` של האפליקציה שלך:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

ודא שיש לך את מאגר Repsy בקובץ `settings.gradle.kts` של הפרויקט שלך:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```