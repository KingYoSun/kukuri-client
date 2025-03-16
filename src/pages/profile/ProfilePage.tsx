import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { useAuthStore } from '@/stores/auth-store';
import { usePostStore } from '@/stores/post-store';
import { useProfileStore } from '@/stores/profile-store';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

const ProfilePage: React.FC = () => {
  const { userId } = useParams<{ userId: string }>();
  const { user } = useAuthStore();
  const { userPosts, fetchUserPosts, isLoading: postsLoading } = usePostStore();
  const { profiles, fetchProfile, updateProfile, followUser, unfollowUser, isLoading: profileLoading } = useProfileStore();
  
  const [isEditing, setIsEditing] = useState(false);
  const [displayName, setDisplayName] = useState('');
  const [bio, setBio] = useState('');
  
  const isOwnProfile = user?.id === userId;
  const profile = userId ? profiles[userId] : null;
  const posts = userId ? userPosts[userId] || [] : [];
  const isFollowing = user && profile ? user.following.includes(profile.id) : false;
  
  useEffect(() => {
    if (userId) {
      fetchProfile(userId);
      fetchUserPosts(userId);
    }
  }, [userId, fetchProfile, fetchUserPosts]);
  
  useEffect(() => {
    if (profile) {
      setDisplayName(profile.displayName);
      setBio(profile.bio || '');
    }
  }, [profile]);
  
  const handleUpdateProfile = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!userId) return;
    
    const success = await updateProfile(userId, {
      displayName,
      bio,
    });
    
    if (success) {
      setIsEditing(false);
    }
  };
  
  const handleFollow = async () => {
    if (!user || !userId) return;
    await followUser(user.id, userId);
  };
  
  const handleUnfollow = async () => {
    if (!user || !userId) return;
    await unfollowUser(user.id, userId);
  };
  
  if (!profile) {
    return (
      <div className="container mx-auto max-w-3xl py-8">
        <div className="text-center">
          {profileLoading ? 'Loading profile...' : 'Profile not found'}
        </div>
      </div>
    );
  }
  
  return (
    <div className="container mx-auto max-w-3xl">
      <div className="bg-card rounded-lg p-6 shadow-sm mb-6">
        {isEditing ? (
          <form onSubmit={handleUpdateProfile} className="space-y-4">
            <div>
              <label htmlFor="displayName" className="block text-sm font-medium mb-1">
                Display Name
              </label>
              <Input
                id="displayName"
                value={displayName}
                onChange={(e) => setDisplayName(e.target.value)}
                required
              />
            </div>
            <div>
              <label htmlFor="bio" className="block text-sm font-medium mb-1">
                Bio
              </label>
              <Input
                id="bio"
                value={bio}
                onChange={(e) => setBio(e.target.value)}
              />
            </div>
            <div className="flex space-x-2">
              <Button type="submit" disabled={profileLoading}>
                Save
              </Button>
              <Button type="button" variant="outline" onClick={() => setIsEditing(false)}>
                Cancel
              </Button>
            </div>
          </form>
        ) : (
          <>
            <div className="flex justify-between items-start">
              <div>
                <h1 className="text-2xl font-bold">{profile.displayName}</h1>
                <p className="text-muted-foreground mt-1">{profile.bio}</p>
              </div>
              {isOwnProfile ? (
                <Button onClick={() => setIsEditing(true)}>Edit Profile</Button>
              ) : (
                <div>
                  {isFollowing ? (
                    <Button variant="outline" onClick={handleUnfollow} disabled={profileLoading}>
                      Unfollow
                    </Button>
                  ) : (
                    <Button onClick={handleFollow} disabled={profileLoading}>
                      Follow
                    </Button>
                  )}
                </div>
              )}
            </div>
            <div className="flex space-x-4 mt-4 text-sm">
              <div>
                <span className="font-medium">{profile.following.length}</span>{' '}
                <span className="text-muted-foreground">Following</span>
              </div>
              <div>
                <span className="font-medium">{profile.followers.length}</span>{' '}
                <span className="text-muted-foreground">Followers</span>
              </div>
            </div>
          </>
        )}
      </div>
      
      <h2 className="text-xl font-semibold mb-4">Posts</h2>
      <div className="space-y-4">
        {postsLoading ? (
          <div className="text-center py-8 text-muted-foreground">Loading posts...</div>
        ) : posts.length === 0 ? (
          <div className="text-center py-8 text-muted-foreground">No posts yet</div>
        ) : (
          posts.map((post) => (
            <div key={post.id} className="bg-card rounded-lg p-4 shadow-sm">
              <div className="flex justify-between items-start mb-2">
                <span className="font-medium">{profile.displayName}</span>
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
          ))
        )}
      </div>
    </div>
  );
};

export default ProfilePage;