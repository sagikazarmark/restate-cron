{ pkgs, ... }:

{
  dotenv.enable = true;

  dagger.enable = true;
  env.DAGGER_X_RELEASE = "v1.0.0-beta.7";

  packages = with pkgs; [
    cargo-release
    cargo-watch
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
