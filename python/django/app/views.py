from django.http import HttpResponse
from django.shortcuts import render
from django.urls import path
from drf_spectacular.utils import extend_schema
from rest_framework.decorators import api_view
from rest_framework.response import Response

# Create your views here.

def index(request):
    return HttpResponse("hello app")

@api_view(['GET'])
def drf_index(request):
    return Response()


routes = [
    path('index/', index),
    path('drf/', drf_index)
]