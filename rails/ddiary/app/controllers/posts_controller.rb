class PostsController < ApplicationController
  def new
  end
  def create
    render text: params[:postu].inspect
  end
end
