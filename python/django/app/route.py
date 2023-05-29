
from django.urls import path, include
from app import views, drf_sample

routes = [
    path('index/', include(views.routes)),
    path('drf_sample/', include(drf_sample.routes))
]