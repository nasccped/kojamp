Contributing
============

> [!NOTE]
>
> This is just a provisional contributing help page. It can change
> soon. So... stay tuned for new contribute guides!

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

## So, how can I contribute?

Simple! With the previous info in mind, I'll give you the
step-by-step to contribute:

1. Forking

Go to the [repository main page](https://github.com/nasccped/kojamp)
and click on **Fork** button.

> Make sure to **uncheck** the `copy only main branch` option!

2. Cloning + set original as upstream

Go to an appropriate path at your machine and clone the repo, then
add the original repo as upstream for later fetching:

```sh
git clone git@github.com:<YOUR_USERNAME>/kojamp
git remote add upstream https://github.com/nasccped/kojamp
```

> [!WARNING]
>
> The command above consider you've forked the repository with the
> default name (`kojamp`). If you have specified a new name, you
> should replace the `kojamp` with the new repository name!

3. Project branch

Before start contributing, go to the target branch that you want to
contribute (`docs`, `docker`, `dev`) and then create a new branch
named by the feature/fix being implemented:

```sh
git checkout <TARGET_BRANCH_NAME> &&
git checkout -b <FEATURE_BRANCH_NAME>
```

4. Changes & committing

Add and commit your changes:

```sh
git add "file_path(s)";
git commit -m "changes commit"
```

> [!TIP]
>
> For commits, you can use the
> [commit conventions cheatsheet](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13#file-conventional-commits-cheatsheet-md).

5. Branch sync

Before pushing your PR, you should sync it to your target branch to
avoid conflict when merging it into `main` branch:

```sh
# fetch + merge from previously mentioned upstream
git fetch upstream <TARGET_BRANCH_NAME> ;
git merge upstream/<TARGET_BRANCH_NAME>

# then, solve conflict cases...
```

6. Pushing PR

And finally, push your pull request to you GitHub forked repo:

```sh
git push origin <FEATURE_BRANCH_NAME>
```

Go to your forked repo at GitHub and set `Base branch` to
`<TARGET_BRANCH_NAME>` and PR it!
