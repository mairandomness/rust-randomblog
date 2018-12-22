cargo build --bin main

sudo systemctl daemon-reload

if sudo systemctl is-active --quiet blog.service; then
    sudo systemctl restart blog.service
else
    sudo systemctl start blog.service
    sudo systemctl enable blog.service
fi

if sudo systemctl is-active --quiet nginx; then
    sudo systemctl restart nginx
else
    sudo systemctl start nginx
    sudo systemctl enable nginx
fi
