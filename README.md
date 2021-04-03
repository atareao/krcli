krcli is a simple tool to work with the keyring from the terminal

If you work with some awesome applications like mutt in the terminal and you want use the  keyring to store your passwords **krcli** is the tool you are looking for.


<h1 align="center">Welcome to gkeyring 👋<h1>

![Licencia MIT](https://img.shields.io/badge/Licencia-MIT-green)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/b3e704c3f150404582cd23b9fcb4be32)](https://www.codacy.com/manual/atareao/gkeyring?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=atareao/gkeyring&amp;utm_campaign=Badge_Grade)
[![CodeFactor](https://www.codefactor.io/repository/github/atareao/gkeyring/badge/master)](https://www.codefactor.io/repository/github/atareao/gkeyring/overview/master)

[![Twitter: atareao](https://img.shields.io/twitter/follow/atareao.svg?style=social)](https://twitter.com/atareao)

<img src="./data/icons/scalable/apps/gkeyring.svg" align="right"
     title="gekeyring Logo" width="128" height="128">


## 🏠 [Homepage](https://www.atareao.es/aplicacion/krcli/)

## Prerequisites

Before you begin, ensure you have met the following requirements:

* If you install it from PPA don't worry about, becouse all the requirements are included in the package
* If you clone the repository, you need, at least, these dependecies,

```
python3
python3-gkeyring
```

## Installing gkeyring

To install **Ubuntu First Steps**, follow these steps:

* In a terminal (`Ctrl+Alt+T`), run these commands

```
sudo add-apt-repository ppa:atareao/atareao
sudo apt update
sudo apt install gkeyring
```

## Using gkeyring

```
Usage: gkeyring [options]

Options:
  -r RING, --ring=RING  the name of the keyring (default "system").
  -g, --get             get password.
  -d, --delete          delete a password.
  -h, --help            show this help and exit.
  -k KEY, --key=KEY     key for the password.
  -p PASSWORD, --password=PASSWORD
                        the password.
  -s, --set             set password.
```

### About this application

![About this app](./screenshots/gkeyring-screenshot-6.png)

## Contributing to gkeyring

To contribute to **gkeyring**, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`
4. Push to the original branch: `git push origin <project_name>/<location>`
5. Create the pull request.

Alternatively see the GitHub documentation on [creating a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

## 👤 Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<table>
  <tr>
    <td align="center"><a href="https://www.atareao.es"><img src="https://avatars3.githubusercontent.com/u/298055?v=4" width="100px;" alt=""/><br /><sub><b>Lorenzo Carbonell</b></sub></a><br /><a href="https://github.com/atareao/gkeyring/commits?author=atareao" title="Code">💻</a></td>
  </tr>
</table>


## Contact

If you want to contact me you can reach me at [atareao.es](https://www.atareao.es).

## License

This project uses the following license: [MIT License](https://choosealicense.com/licenses/mit/).
