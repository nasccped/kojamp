Contributing
============

Do you want to contribute in **kojamp** project? I really appreciate
it ❤️

Let's talk about some things that you should know before
contributing!

This project is separated in branches where each one have it own
reason to exists:

- `docs`: stores the project documentation (readme's);
- `docker`: stores the **Docker** related content;
- `dev`: stores the project source code (rust files + `.toml`);
- `scripts`: stores the project scripts for deploying
- `main`: latest **stable** changes (ready to deploy)
- `unstable`: latest **non-stable** changes

<div align="center">

| branch     | can it change? | why?                                                                   |
| :--------- | :------------- | :--------------------------------------------------------------------- |
| `docs`     | ✅ Yes         | The documentation can contain some misinformation / be improved        |
| `docker`   | ✅ Yes         | The Docker build process can be improved to a more lighweight strategy |
| `dev`      | ✅ Yes         | It's literally the source code 😐                                      |
| `scripts`  | ❌ No          | It's for my personal usage, no improvements required                   |
| `main`     | ❌ No          | Changes shouldn't be done at main. It can break the application        |
| `unstable` | ❌ No          | No changes here. It can result in conflicts when merging               |

</div>
