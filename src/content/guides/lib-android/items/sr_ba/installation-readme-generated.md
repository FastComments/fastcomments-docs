Dodajte FastComments SDK u datoteku `build.gradle.kts` vaše aplikacije:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Provjerite da li imate Repsy repozitorij u datoteci `settings.gradle.kts` vašeg projekta:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // ostali repozitoriji...
    }
}
```