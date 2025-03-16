import React, { useEffect, useState } from 'react';
import { useAuthStore } from '@/stores/auth-store';
import { usePostStore, Post } from '@/stores/post-store';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useProfileStore } from '@/stores/profile-store';
import { Link } from 'react-router-dom';

const HomePage: React.FC = () => {
  const { user } = useAuthStore();
  const { posts, fetchPosts, createPost, isLoading } = usePostStore();
  const { profiles, fetchProfile } = useProfileStore();
  const [newPostContent, setNewPostContent] = useState('');

  useEffect(() => {
    fetchPosts();
  }, [fetchPosts]);

  // Fetch profiles for post authors
  useEffect(() => {
    const fetchMissingProfiles = async () => {
      const uniqueAuthorIds = [...new Set(posts.map(post => post.authorId))];
      
      for (const authorId of uniqueAuthorIds) {
        if (!profiles[authorId]) {
          await fetchProfile(authorId);
        }
      }
    };

    if (posts.length > 0) {
      fetchMissingProfiles();
    }
  }, [posts, profiles, fetchProfile]);

  const handleCreatePost = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newPostContent.trim()) return;
    
    await createPost(newPostContent);
    setNewPostContent('');
  };

  // タイムスタンプをフォーマットする関数（投稿カードで使用）

  return (
    <div className="container mx-auto max-w-3xl">
      <h1 className="text-2xl font-bold mb-6">Home Timeline</h1>
      
      {user && (
        <div className="bg-card rounded-lg p-4 mb-6 shadow-sm">
          <form onSubmit={handleCreatePost}>
            <div className="flex flex-col space-y-2">
              <Input
                placeholder="What's on your mind?"
                value={newPostContent}
                onChange={(e) => setNewPostContent(e.target.value)}
                disabled={isLoading}
                className="min-h-[80px] resize-none"
              />
              <div className="flex justify-end">
                <Button type="submit" disabled={isLoading || !newPostContent.trim()}>
                  {isLoading ? 'Posting...' : 'Post'}
                </Button>
              </div>
            </div>
          </form>
        </div>
      )}
      
      <div className="space-y-4">
        {posts.length === 0 ? (
          <div className="text-center py-8 text-muted-foreground">
            No posts yet. Be the first to post something!
          </div>
        ) : (
          posts.map((post) => (
            <PostCard 
              key={post.id} 
              post={post} 
              authorName={profiles[post.authorId]?.displayName || 'Unknown User'} 
            />
          ))
        )}
      </div>
    </div>
  );
};

interface PostCardProps {
  post: Post;
  authorName: string;
}

const PostCard: React.FC<PostCardProps> = ({ post, authorName }) => {
  return (
    <div className="bg-card rounded-lg p-4 shadow-sm">
      <div className="flex justify-between items-start mb-2">
        <Link to={`/profile/${post.authorId}`} className="font-medium hover:underline">
          {authorName}
        </Link>
        <span className="text-xs text-muted-foreground">
          {new Date(post.createdAt).toLocaleString()}
        </span>
      </div>
      <p className="whitespace-pre-wrap">{post.content}</p>
      
      {post.hashtags.length > 0 && (
        <div className="mt-2 flex flex-wrap gap-1">
          {post.hashtags.map((tag) => (
            <span key={tag} className="text-sm text-blue-500">
              #{tag}
            </span>
          ))}
        </div>
      )}
    </div>
  );
};

export default HomePage;