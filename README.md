# keep-in-touch (kit)

![Rust](https://github.com/nickmachnik/keep-in-touch/workflows/Rust/badge.svg)
![Travis-CI](https://travis-ci.org/nickmachnik/keep-in-touch.svg?branch=master)

`kit` is a very simple and fast command line tool that helps you keeping in touch with your friends and family. It maintains a table of friends, contact intervals and last dates of contact, which you can view in the command line. The viewing features automatic sorting of the table by the next contacts, as well as colour coding by urgency.

## Installing

### Linux

Download the latest [release](https://github.com/nickmachnik/keep-in-touch/releases/latest) to a directory in which you would like to keep the binary.
For example:

```
cd
mkdir ./.keep_in_touch
cd .keep_in_touch
wget https://github.com/nickmachnik/keep-in-touch/releases/download/v0.1.1/keep-in-touch-v0.1.1-x86_64-unknown-linux-gnu.tar.gz
tar -xf keep-in-touch-v0.2.2-x86_64-unknown-linux-musl.tar.gz
```

Add that directory to your path. On Ubuntu you could add this line to your `.bashrc`:

```
export PATH="${HOME}/.keep_in_touch:$PATH"
```

If you would like to enable bash autocompletion for `kit`, source the autocompletion script on shell start by adding this to your `.bashrc`:

```
source ${HOME}/.keep_in_touch/kit-completion.sh
```

Unfortunately, autocompletion does not work properly in zsh at the moment.

## Usage

```
kit -h
```

## Autocompletions

`kit` supports bash autocompletions, including the names saved in your table of friends.
These are updated everytime you add a name via `kit add` or modify one via `kit modify`.
If you would like to do an update without any modification of your table, you can also
use `kit update-autocompletion`.
This changes the 'kit-completion.sh' that is by default stored in the same directory as the 'kit' binary exectuable.
Before these changes come into action, that scripts needs to be sourced from within you shell with `source` (see [Installing](#Installing)).

## License

Double licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Acknowledgments

This project is largely inspired by an idea (and habit) of [Martin Frassek's](https://github.com/MFrassek).

<!-- 
End with an example of getting some data out of the system or using it for a little demo

## Running the tests

Explain how to run the automated tests for this system

### Break down into end to end tests

Explain what these tests test and why

```
Give an example
```

### And coding style tests

Explain what these tests test and why

```
Give an example
```

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

* [Dropwizard](http://www.dropwizard.io/1.0.2/docs/) - The web framework used
* [Maven](https://maven.apache.org/) - Dependency Management
* [ROME](https://rometools.github.io/rome/) - Used to generate RSS Feeds

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags).

## Authors

* **Billie Thompson** - *Initial work* - [PurpleBooth](https://github.com/PurpleBooth)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Hat tip to anyone whose code was used
* Inspiration
* etc

 -->
