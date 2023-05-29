
from django.urls import path, include
from app import views

routes = [
    path('index/', include(views.routes))
]