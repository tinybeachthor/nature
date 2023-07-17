{
  inputs = {
    dream2nix.url = "github:nix-community/dream2nix/3802b906b4107e2d8bedc292d9c43779c4b4ec47";
  };
  outputs = { self, dream2nix }:
    dream2nix.lib.makeFlakeOutputs {
      systems = ["x86_64-linux"];
      config.projectRoot = ./.;
      source = ./.;
      projects = ./projects.toml;
    };
}
