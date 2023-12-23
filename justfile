[no-exit-message]
[private]
@default:
    if [ -z "fzf" ]; then \
        just -l && exit 0; \
    else \
        recipe=$(just -l --list-heading '' --list-prefix '' | fzf | awk '{print $1}'); \
        if [ -n "$recipe" ]; then \
            just "$recipe"; \
        else \
            echo "No recipe selected"; \
        fi \
    fi

edit:
  $EDITOR ./justfile

watch:
  cargo leptos watch
