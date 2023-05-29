from django.http import HttpResponse
from django.shortcuts import render
from django.urls import path
from drf_spectacular.utils import extend_schema

# Create your views here.

@extend_schema(
    responses={200: "ok"}
)
def index(request):
    return HttpResponse("hello app")


routes = [
    path('', index)
]